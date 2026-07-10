use std::{
    collections::{BTreeMap, HashMap},
    env, fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context as _, Result, bail};
use clap::{Args, Parser, Subcommand};
use octocrab::{Octocrab, params};
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use walkdir::WalkDir;

const DEFAULT_MANIFEST_URL: &str = "https://static.rust-lang.org/dist/channel-rust-stable.toml";
const USER_AGENT_VALUE: &str = "skills-rust-stable-check";

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    CheckRustStable(CheckRustStableArgs),
}

#[derive(Args, Debug)]
struct CheckRustStableArgs {
    #[arg(
        long,
        env = "RUST_STABLE_MANIFEST",
        default_value = DEFAULT_MANIFEST_URL
    )]
    manifest_url: String,

    #[arg(long, default_value = "skills")]
    skills_root: PathBuf,

    #[arg(long)]
    create_issue: bool,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct RustMinorVersion {
    major: u64,
    minor: u64,
}

impl FromStr for RustMinorVersion {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        let mut parts = value.split('.');
        let major = parts.next().unwrap_or_default();
        let Some(minor) = parts.next() else {
            bail!("missing Rust minor version in `{value}`");
        };
        if let Some(patch) = parts.next() {
            patch
                .parse::<u64>()
                .with_context(|| format!("invalid Rust patch version in `{value}`"))?;
        }
        if parts.next().is_some() {
            bail!("Rust version must be in `major.minor` or `major.minor.patch` form: `{value}`");
        }

        Ok(Self {
            major: major
                .parse()
                .with_context(|| format!("invalid Rust major version in `{value}`"))?,
            minor: minor
                .parse()
                .with_context(|| format!("invalid Rust minor version in `{value}`"))?,
        })
    }
}

impl std::fmt::Display for RustMinorVersion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}.{}", self.major, self.minor)
    }
}

#[derive(Debug)]
struct RustSyncState {
    latest_full: String,
    latest_minor: RustMinorVersion,
    manifest_date: String,
    tracked_minor: Option<RustMinorVersion>,
    tracked_files: Vec<String>,
}

impl RustSyncState {
    fn should_open_issue(&self) -> bool {
        self.tracked_minor
            .is_some_and(|tracked_minor| self.latest_minor > tracked_minor)
    }

    fn issue_title(&self) -> String {
        format!("sync skills: rust {}", self.latest_minor)
    }
}

#[derive(Debug, Deserialize)]
struct ChannelManifest {
    date: Option<String>,
    pkg: HashMap<String, ManifestPackage>,
}

#[derive(Debug, Deserialize)]
struct ManifestPackage {
    version: String,
}

#[derive(Debug)]
struct Repository {
    owner: String,
    name: String,
}

impl FromStr for Repository {
    type Err = anyhow::Error;

    fn from_str(repository: &str) -> Result<Self> {
        let Some((owner, name)) = repository.split_once('/') else {
            bail!("GitHub repository must be in `owner/repo` form");
        };

        if owner.is_empty() || name.is_empty() || name.contains('/') {
            bail!("GitHub repository must be in `owner/repo` form");
        }

        Ok(Self {
            owner: owner.to_owned(),
            name: name.to_owned(),
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::CheckRustStable(args) => check_rust_stable(&args).await,
    }
}

async fn check_rust_stable(args: &CheckRustStableArgs) -> Result<()> {
    let client = Client::builder()
        .user_agent(USER_AGENT_VALUE)
        .build()
        .context("failed to build HTTP client")?;
    let state = build_sync_state(&client, &args.manifest_url, &args.skills_root).await?;

    let result = if !state.should_open_issue() {
        "No sync issue needed.".to_owned()
    } else if !args.create_issue {
        format!("Would open issue: {}", state.issue_title())
    } else {
        let repository = env::var("GITHUB_REPOSITORY")
            .context("GITHUB_REPOSITORY is required to create issues")?;
        let token =
            env::var("GITHUB_TOKEN").context("GITHUB_TOKEN is required to create issues")?;
        let api_url =
            env::var("GITHUB_API_URL").unwrap_or_else(|_| "https://api.github.com".to_owned());
        let repository = repository.parse::<Repository>()?;
        let github = GitHubClient::new(api_url, repository, token)?;
        github.open_sync_issue(&state).await?
    };

    print_result(&state, &result);
    append_step_summary(
        env::var_os("GITHUB_STEP_SUMMARY").as_deref(),
        &state,
        &result,
    )?;
    Ok(())
}

async fn build_sync_state(
    client: &Client,
    manifest_url: &str,
    skills_root: &Path,
) -> Result<RustSyncState> {
    let manifest = fetch_manifest(client, manifest_url).await?;
    let rust_package = manifest
        .pkg
        .get("rust")
        .context("Rust stable manifest is missing `pkg.rust`")?;
    let latest_full = rust_package
        .version
        .split_whitespace()
        .next()
        .context("Rust stable manifest has an empty `pkg.rust.version`")?
        .to_owned();
    let latest_minor = latest_full.parse::<RustMinorVersion>()?;

    let tracked_versions = scan_tracked_rust_versions(skills_root)?;
    let (tracked_minor, tracked_files) = tracked_versions
        .last_key_value()
        .map(|(version, files)| (Some(*version), files.to_vec()))
        .unwrap_or((None, Vec::new()));

    Ok(RustSyncState {
        latest_full,
        latest_minor,
        manifest_date: manifest.date.unwrap_or_else(|| "unknown".to_owned()),
        tracked_minor,
        tracked_files,
    })
}

async fn fetch_manifest(client: &Client, manifest_url: &str) -> Result<ChannelManifest> {
    let body = client
        .get(manifest_url)
        .send()
        .await
        .with_context(|| format!("failed to fetch Rust stable manifest from {manifest_url}"))?
        .error_for_status()
        .with_context(|| format!("Rust stable manifest request failed for {manifest_url}"))?
        .text()
        .await
        .context("failed to read Rust stable manifest response body")?;

    toml::from_str(&body).context("failed to parse Rust stable manifest TOML")
}

fn scan_tracked_rust_versions(
    skills_root: &Path,
) -> Result<BTreeMap<RustMinorVersion, Vec<String>>> {
    let mut tracked_versions: BTreeMap<RustMinorVersion, Vec<String>> = BTreeMap::new();

    if !skills_root.exists() {
        return Ok(tracked_versions);
    }

    let rust_version_pattern = Regex::new(r"(?i)\brust\s+([0-9]+\.[0-9]+)(?:\.[0-9]+)?\b")
        .context("failed to compile Rust version regex")?;

    for entry in WalkDir::new(skills_root) {
        let entry = entry.with_context(|| format!("failed to walk {}", skills_root.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let Ok(text) = fs::read_to_string(path) else {
            continue;
        };

        for captures in rust_version_pattern.captures_iter(&text) {
            let version = captures[1].parse::<RustMinorVersion>()?;
            let path = path.to_string_lossy().replace('\\', "/");
            tracked_versions.entry(version).or_default().push(path);
        }
    }

    for files in tracked_versions.values_mut() {
        files.sort();
        files.dedup();
    }

    Ok(tracked_versions)
}

fn build_issue_body(state: &RustSyncState) -> String {
    let mut body = vec![
        format!(
            "The Rust stable channel is now **Rust {}** (manifest date: `{}`).",
            state.latest_full, state.manifest_date
        ),
        String::new(),
    ];

    if let Some(tracked_minor) = state.tracked_minor {
        body.extend([
            format!(
                "The highest Rust baseline currently tracked in `skills/` is **Rust {tracked_minor}**."
            ),
            String::new(),
            "Files mentioning that baseline:".to_owned(),
        ]);
        body.extend(
            state
                .tracked_files
                .iter()
                .map(|tracked_file| format!("- `{tracked_file}`")),
        );
        body.push(String::new());
    } else {
        body.extend([
            "No tracked Rust baseline was detected in `skills/`.".to_owned(),
            String::new(),
        ]);
    }

    body.extend([
        "Suggested follow-up:".to_owned(),
        format!(
            "- Review Rust {} release notes and update the Rust skills that track the latest stable toolchain.",
            state.latest_minor
        ),
        "- Refresh any version-specific guidance, examples, and agent descriptions that mention the previous baseline."
            .to_owned(),
    ]);

    body.join("\n") + "\n"
}

fn print_result(state: &RustSyncState, result: &str) {
    println!("Latest Rust stable: {}", state.latest_full);
    println!(
        "Tracked Rust baseline: {}",
        state
            .tracked_minor
            .map_or_else(|| "not found".to_owned(), |version| version.to_string())
    );
    println!("{result}");
}

fn append_step_summary(
    step_summary_path: Option<&std::ffi::OsStr>,
    state: &RustSyncState,
    result: &str,
) -> Result<()> {
    let Some(path) = step_summary_path else {
        return Ok(());
    };

    let mut summary = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .with_context(|| {
            format!(
                "failed to open GitHub step summary at {}",
                Path::new(path).display()
            )
        })?;

    use std::io::Write as _;
    let tracked_minor = state
        .tracked_minor
        .map_or_else(|| "not found".to_owned(), |version| version.to_string());
    let content = format!(
        "Latest Rust stable: **{}**\n\nTracked Rust baseline: **{tracked_minor}**\n\n{result}\n",
        state.latest_full
    );
    summary.write_all(content.as_bytes())?;

    Ok(())
}

struct GitHubClient {
    client: Octocrab,
    repository: Repository,
}

impl GitHubClient {
    fn new(api_url: String, repository: Repository, token: String) -> Result<Self> {
        let client = Octocrab::builder()
            .personal_token(token)
            .base_uri(api_url)
            .context("failed to configure GitHub API base URI")?
            .build()
            .context("failed to build GitHub API client")?;

        Ok(Self { client, repository })
    }

    async fn open_sync_issue(&self, state: &RustSyncState) -> Result<String> {
        let title = state.issue_title();
        let existing =
            self.list_open_issues().await?.into_iter().find(|issue| {
                issue.pull_request.is_none() && issue.title.eq_ignore_ascii_case(&title)
            });

        if let Some(issue) = existing {
            return Ok(format!("Open issue already exists: {}", issue.html_url));
        }

        let issue = self
            .client
            .issues(&self.repository.owner, &self.repository.name)
            .create(&title)
            .body(build_issue_body(state))
            .send()
            .await
            .context("failed to create GitHub issue")?;
        Ok(format!("Created issue: {}", issue.html_url))
    }

    async fn list_open_issues(&self) -> Result<Vec<octocrab::models::issues::Issue>> {
        let page = self
            .client
            .issues(&self.repository.owner, &self.repository.name)
            .list()
            .state(params::State::Open)
            .per_page(100)
            .send()
            .await
            .context("failed to list open GitHub issues")?;

        self.client
            .all_pages(page)
            .await
            .context("failed to paginate open GitHub issues")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_minor_versions() {
        assert!("1.98.0".parse::<RustMinorVersion>().unwrap() > "1.97".parse().unwrap());
        assert_eq!(
            "1.97.1".parse::<RustMinorVersion>().unwrap().to_string(),
            "1.97"
        );
    }

    #[test]
    fn rejects_malformed_minor_versions() {
        for version in ["1", "stable.97", "1.97.beta", "1.97.0.1", "1..97"] {
            assert!(
                version.parse::<RustMinorVersion>().is_err(),
                "{version} should be rejected"
            );
        }
    }

    #[test]
    fn detects_when_sync_issue_is_needed() {
        let state = RustSyncState {
            latest_full: "1.98.0".to_owned(),
            latest_minor: "1.98".parse().unwrap(),
            manifest_date: "2026-08-20".to_owned(),
            tracked_minor: Some("1.97".parse().unwrap()),
            tracked_files: vec!["skills/rust-test/SKILL.md".to_owned()],
        };

        assert!(state.should_open_issue());
        assert_eq!(state.issue_title(), "sync skills: rust 1.98");
    }

    #[test]
    fn patch_release_does_not_require_sync_issue() {
        let state = RustSyncState {
            latest_full: "1.97.1".to_owned(),
            latest_minor: "1.97.1".parse().unwrap(),
            manifest_date: "2026-07-16".to_owned(),
            tracked_minor: Some("1.97".parse().unwrap()),
            tracked_files: vec!["skills/rust-test/SKILL.md".to_owned()],
        };

        assert!(!state.should_open_issue());
    }

    #[test]
    fn scans_rust_baselines_case_insensitively() {
        let tempdir = tempfile::tempdir().unwrap();
        let skills = tempdir.path().join("skills");
        fs::create_dir(&skills).unwrap();
        fs::write(skills.join("one.md"), "Assume Rust 1.97 stable.").unwrap();
        fs::write(skills.join("two.md"), "use rust 1.98 guidance").unwrap();

        let tracked_versions = scan_tracked_rust_versions(&skills).unwrap();

        assert!(tracked_versions.contains_key(&"1.97".parse().unwrap()));
        assert!(tracked_versions.contains_key(&"1.98".parse().unwrap()));
    }

    #[test]
    fn missing_skills_root_has_no_tracked_versions() {
        let tempdir = tempfile::tempdir().unwrap();
        let missing_root = tempdir.path().join("missing");

        assert!(
            scan_tracked_rust_versions(&missing_root)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn scan_skips_files_that_are_not_utf8() {
        let tempdir = tempfile::tempdir().unwrap();
        fs::write(tempdir.path().join("binary"), [0xff, 0xfe]).unwrap();

        assert!(
            scan_tracked_rust_versions(tempdir.path())
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn issue_body_lists_tracked_files() {
        let state = RustSyncState {
            latest_full: "1.98.0".to_owned(),
            latest_minor: "1.98".parse().unwrap(),
            manifest_date: "2026-08-20".to_owned(),
            tracked_minor: Some("1.97".parse().unwrap()),
            tracked_files: vec!["skills/rust-test/SKILL.md".to_owned()],
        };

        let body = build_issue_body(&state);

        assert!(body.contains("The Rust stable channel is now **Rust 1.98.0**"));
        assert!(body.contains("- `skills/rust-test/SKILL.md`"));
    }

    #[test]
    fn issue_body_explains_when_no_baseline_is_tracked() {
        let state = RustSyncState {
            latest_full: "1.98.0".to_owned(),
            latest_minor: "1.98".parse().unwrap(),
            manifest_date: "2026-08-20".to_owned(),
            tracked_minor: None,
            tracked_files: Vec::new(),
        };

        let body = build_issue_body(&state);

        assert!(body.contains("No tracked Rust baseline was detected in `skills/`."));
    }

    #[test]
    fn step_summary_reports_missing_baseline() {
        let tempdir = tempfile::tempdir().unwrap();
        let summary_path = tempdir.path().join("summary.md");
        let state = RustSyncState {
            latest_full: "1.98.0".to_owned(),
            latest_minor: "1.98".parse().unwrap(),
            manifest_date: "2026-08-20".to_owned(),
            tracked_minor: None,
            tracked_files: Vec::new(),
        };

        append_step_summary(
            Some(summary_path.as_os_str()),
            &state,
            "No sync issue needed.",
        )
        .unwrap();

        let summary = fs::read_to_string(summary_path).unwrap();
        assert!(summary.contains("Tracked Rust baseline: **not found**"));
    }

    #[test]
    fn step_summary_open_errors_include_the_path() {
        let tempdir = tempfile::tempdir().unwrap();
        let state = RustSyncState {
            latest_full: "1.98.0".to_owned(),
            latest_minor: "1.98".parse().unwrap(),
            manifest_date: "2026-08-20".to_owned(),
            tracked_minor: None,
            tracked_files: Vec::new(),
        };

        let error = append_step_summary(
            Some(tempdir.path().as_os_str()),
            &state,
            "No sync issue needed.",
        )
        .unwrap_err();

        assert!(error.to_string().contains(&format!(
            "failed to open GitHub step summary at {}",
            tempdir.path().display()
        )));
    }

    #[test]
    fn parses_repository_from_github_actions_environment_shape() {
        let repository = "owner/repo".parse::<Repository>().unwrap();

        assert_eq!(repository.owner, "owner");
        assert_eq!(repository.name, "repo");
    }

    #[test]
    fn rejects_malformed_repository() {
        assert!("owner".parse::<Repository>().is_err());
        assert!("owner/repo/extra".parse::<Repository>().is_err());
        assert!("/repo".parse::<Repository>().is_err());
        assert!("owner/".parse::<Repository>().is_err());
    }

    #[tokio::test]
    async fn github_client_does_not_duplicate_api_version_header() {
        use std::{
            io::{Read as _, Write as _},
            net::TcpListener,
            thread,
            time::Duration,
        };

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let api_url = format!("http://{}", listener.local_addr().unwrap());
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();

            let mut request = Vec::new();
            let mut buffer = [0; 4096];
            while !request.windows(4).any(|window| window == b"\r\n\r\n") {
                let bytes_read = stream.read(&mut buffer).unwrap();
                assert_ne!(bytes_read, 0, "request ended before the headers arrived");
                request.extend_from_slice(&buffer[..bytes_read]);
            }

            stream
                .write_all(
                    b"HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: 2\r\nconnection: close\r\n\r\n[]",
                )
                .unwrap();

            String::from_utf8(request).unwrap()
        });

        let github = GitHubClient::new(
            api_url,
            Repository {
                owner: "owner".to_owned(),
                name: "repo".to_owned(),
            },
            "test-token".to_owned(),
        )
        .unwrap();

        assert!(github.list_open_issues().await.unwrap().is_empty());

        let request = server.join().unwrap();
        let api_version_headers = request
            .lines()
            .filter_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("x-github-api-version")
                    .then_some(value.trim())
            })
            .collect::<Vec<_>>();

        assert!(api_version_headers.len() <= 1, "request was:\n{request}");
        assert!(
            api_version_headers.iter().all(|value| !value.contains(',')),
            "request was:\n{request}"
        );
    }
}

use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::{Context as _, Result};
use reqwest::Client;

mod channel;
mod github;
mod report;
mod versions;

use channel::fetch_manifest;
use github::{GitHubClient, Repository};
use report::{append_step_summary, print_result};
use versions::{RustMinorVersion, scan_tracked_rust_versions};

const DEFAULT_MANIFEST_URL: &str = "https://static.rust-lang.org/dist/channel-rust-stable.toml";
const USER_AGENT_VALUE: &str = "skills-rust-stable-check";

#[derive(clap::Args, Debug)]
pub(crate) struct Args {
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

pub(crate) async fn run(args: &Args) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}

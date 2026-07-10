use std::{
    fs,
    io::Write as _,
    net::TcpListener,
    path::Path,
    process::{Command, Output},
    thread,
    time::Duration,
};

struct TestResponse {
    status: &'static str,
    body: String,
}

impl TestResponse {
    fn ok(body: impl Into<String>) -> Self {
        Self {
            status: "200 OK",
            body: body.into(),
        }
    }
}

fn spawn_server(responses: Vec<TestResponse>) -> (String, thread::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());
    let server = thread::spawn(move || {
        let mut requests = Vec::new();

        for response in responses {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();

            let request = read_request(&mut stream);
            requests.push(String::from_utf8(request).unwrap());

            write!(
                stream,
                "HTTP/1.1 {}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                response.status,
                response.body.len(),
                response.body
            )
            .unwrap();
        }

        requests
    });

    (base_url, server)
}

fn read_request(stream: &mut impl std::io::Read) -> Vec<u8> {
    let mut request = Vec::new();
    let mut buffer = [0; 4096];
    let header_end = loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        assert_ne!(bytes_read, 0, "request ended before the headers arrived");
        request.extend_from_slice(&buffer[..bytes_read]);
        if let Some(header_end) = request.windows(4).position(|window| window == b"\r\n\r\n") {
            break header_end + 4;
        }
    };

    let headers = String::from_utf8_lossy(&request[..header_end]);
    let content_length = headers
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().unwrap())
        })
        .unwrap_or(0);

    while request.len() < header_end + content_length {
        let bytes_read = stream.read(&mut buffer).unwrap();
        assert_ne!(bytes_read, 0, "request ended before the body arrived");
        request.extend_from_slice(&buffer[..bytes_read]);
    }

    request
}

fn stable_manifest(version: &str, date: Option<&str>) -> String {
    let date = date.map_or_else(String::new, |date| format!("date = \"{date}\"\n"));
    format!("{date}[pkg.rust]\nversion = \"{version} (test hash)\"\n")
}

fn issue_json(base_url: &str, title: &str) -> String {
    format!(
        r#"{{
            "id": 1,
            "node_id": "I_1",
            "url": "{base_url}/repos/owner/repo/issues/1",
            "repository_url": "{base_url}/repos/owner/repo",
            "labels_url": "{base_url}/repos/owner/repo/issues/1/labels",
            "comments_url": "{base_url}/repos/owner/repo/issues/1/comments",
            "events_url": "{base_url}/repos/owner/repo/issues/1/events",
            "html_url": "{base_url}/owner/repo/issues/1",
            "number": 1,
            "state": "open",
            "state_reason": null,
            "title": "{title}",
            "body": null,
            "user": {{
                "login": "test-user",
                "id": 1,
                "node_id": "U_1",
                "avatar_url": "{base_url}/avatar",
                "gravatar_id": "",
                "url": "{base_url}/users/test-user",
                "html_url": "{base_url}/test-user",
                "followers_url": "{base_url}/users/test-user/followers",
                "following_url": "{base_url}/users/test-user/following",
                "gists_url": "{base_url}/users/test-user/gists",
                "starred_url": "{base_url}/users/test-user/starred",
                "subscriptions_url": "{base_url}/users/test-user/subscriptions",
                "organizations_url": "{base_url}/users/test-user/orgs",
                "repos_url": "{base_url}/users/test-user/repos",
                "events_url": "{base_url}/users/test-user/events",
                "received_events_url": "{base_url}/users/test-user/received_events",
                "type": "User",
                "site_admin": false,
                "name": null,
                "patch_url": null
            }},
            "labels": [],
            "assignee": null,
            "assignees": [],
            "author_association": null,
            "milestone": null,
            "locked": false,
            "active_lock_reason": null,
            "comments": 0,
            "pull_request": null,
            "closed_at": null,
            "closed_by": null,
            "created_at": "2026-07-10T00:00:00Z",
            "updated_at": "2026-07-10T00:00:00Z"
        }}"#
    )
}

fn write_tracked_skill(root: &Path, version: &str) {
    fs::create_dir_all(root).unwrap();
    fs::write(
        root.join("SKILL.md"),
        format!("Assume Rust {version} stable."),
    )
    .unwrap();
}

fn xtask(manifest_url: &str, skills_root: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_xtask"));
    command
        .arg("check-rust-stable")
        .arg("--manifest-url")
        .arg(manifest_url)
        .arg("--skills-root")
        .arg(skills_root)
        .env_remove("GITHUB_STEP_SUMMARY")
        .env_remove("GITHUB_REPOSITORY")
        .env_remove("GITHUB_TOKEN")
        .env_remove("GITHUB_API_URL");
    command
}

fn assert_success(output: &Output) -> String {
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout.clone()).unwrap()
}

#[test]
fn reports_no_sync_needed_and_writes_step_summary() {
    let tempdir = tempfile::tempdir().unwrap();
    let skills_root = tempdir.path().join("skills");
    let summary_path = tempdir.path().join("summary.md");
    write_tracked_skill(&skills_root, "1.97");
    let (base_url, server) = spawn_server(vec![TestResponse::ok(stable_manifest(
        "1.97.1",
        Some("2026-07-10"),
    ))]);

    let output = xtask(&format!("{base_url}/manifest"), &skills_root)
        .env("GITHUB_STEP_SUMMARY", &summary_path)
        .output()
        .unwrap();

    let stdout = assert_success(&output);
    assert!(stdout.contains("Latest Rust stable: 1.97.1"));
    assert!(stdout.contains("Tracked Rust baseline: 1.97"));
    assert!(stdout.contains("No sync issue needed."));
    assert!(
        fs::read_to_string(summary_path)
            .unwrap()
            .contains("No sync issue needed.")
    );
    let requests = server.join().unwrap();
    assert!(requests[0].starts_with("GET /manifest HTTP/1.1"));
}

#[test]
fn reports_pending_sync_without_creating_an_issue() {
    let tempdir = tempfile::tempdir().unwrap();
    let skills_root = tempdir.path().join("skills");
    write_tracked_skill(&skills_root, "1.97");
    let (base_url, server) = spawn_server(vec![TestResponse::ok(stable_manifest("1.98.0", None))]);

    let output = xtask(&format!("{base_url}/manifest"), &skills_root)
        .output()
        .unwrap();

    let stdout = assert_success(&output);
    assert!(stdout.contains("Would open issue: sync skills: rust 1.98"));
    server.join().unwrap();
}

#[test]
fn reports_when_no_rust_baseline_is_tracked() {
    let tempdir = tempfile::tempdir().unwrap();
    let missing_skills_root = tempdir.path().join("missing-skills");
    let (base_url, server) = spawn_server(vec![TestResponse::ok(stable_manifest(
        "1.97.1",
        Some("2026-07-10"),
    ))]);

    let output = xtask(&format!("{base_url}/manifest"), &missing_skills_root)
        .output()
        .unwrap();

    let stdout = assert_success(&output);
    assert!(stdout.contains("Tracked Rust baseline: not found"));
    assert!(stdout.contains("No sync issue needed."));
    server.join().unwrap();
}

#[test]
fn reports_manifest_request_and_parse_failures() {
    let tempdir = tempfile::tempdir().unwrap();
    let skills_root = tempdir.path().join("skills");
    write_tracked_skill(&skills_root, "1.97");

    for (response, expected_error) in [
        (
            TestResponse {
                status: "500 Internal Server Error",
                body: "server error".to_owned(),
            },
            "Rust stable manifest request failed",
        ),
        (
            TestResponse::ok("not valid = ["),
            "failed to parse Rust stable manifest TOML",
        ),
        (
            TestResponse::ok("[pkg.other]\nversion = \"1.98.0\"\n"),
            "Rust stable manifest is missing `pkg.rust`",
        ),
        (
            TestResponse::ok("[pkg.rust]\nversion = \"\"\n"),
            "Rust stable manifest has an empty `pkg.rust.version`",
        ),
        (
            TestResponse::ok("[pkg.rust]\nversion = \"stable\"\n"),
            "missing Rust minor version",
        ),
    ] {
        let (base_url, server) = spawn_server(vec![response]);
        let output = xtask(&format!("{base_url}/manifest"), &skills_root)
            .output()
            .unwrap();

        assert!(!output.status.success());
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(expected_error),
            "stderr did not contain {expected_error:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        server.join().unwrap();
    }

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let unavailable_url = format!("http://{}/manifest", listener.local_addr().unwrap());
    drop(listener);
    let output = xtask(&unavailable_url, &skills_root).output().unwrap();

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("failed to fetch Rust stable manifest")
    );
}

#[test]
fn reports_step_summary_write_failures() {
    let tempdir = tempfile::tempdir().unwrap();
    let skills_root = tempdir.path().join("skills");
    write_tracked_skill(&skills_root, "1.97");
    let (base_url, server) = spawn_server(vec![TestResponse::ok(stable_manifest("1.97.1", None))]);

    let output = xtask(&format!("{base_url}/manifest"), &skills_root)
        .env("GITHUB_STEP_SUMMARY", tempdir.path())
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("failed to open GitHub step summary"));
    server.join().unwrap();
}

#[test]
fn reuses_an_existing_sync_issue() {
    let tempdir = tempfile::tempdir().unwrap();
    let skills_root = tempdir.path().join("skills");
    write_tracked_skill(&skills_root, "1.97");
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());
    let existing_issue = issue_json(&base_url, "Sync Skills: Rust 1.98");
    let server = thread::spawn(move || {
        let responses = [
            stable_manifest("1.98.0", Some("2026-08-20")),
            format!("[{existing_issue}]"),
        ];
        let mut requests = Vec::new();
        for body in responses {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            requests.push(String::from_utf8(read_request(&mut stream)).unwrap());
            write!(
                stream,
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                body.len()
            )
            .unwrap();
        }
        requests
    });

    let output = xtask(&format!("{base_url}/manifest"), &skills_root)
        .arg("--create-issue")
        .env("GITHUB_REPOSITORY", "owner/repo")
        .env("GITHUB_TOKEN", "test-token")
        .env("GITHUB_API_URL", &base_url)
        .output()
        .unwrap();

    let stdout = assert_success(&output);
    assert!(stdout.contains(&format!(
        "Open issue already exists: {base_url}/owner/repo/issues/1"
    )));
    let requests = server.join().unwrap();
    assert!(requests[1].starts_with("GET /repos/owner/repo/issues?"));
}

#[test]
fn creates_a_sync_issue_when_none_exists() {
    let tempdir = tempfile::tempdir().unwrap();
    let skills_root = tempdir.path().join("skills");
    write_tracked_skill(&skills_root, "1.97");
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());
    let created_issue = issue_json(&base_url, "sync skills: rust 1.98");
    let server = thread::spawn(move || {
        let responses = [
            stable_manifest("1.98.0", Some("2026-08-20")),
            "[]".to_owned(),
            created_issue,
        ];
        let mut requests = Vec::new();
        for body in responses {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            requests.push(String::from_utf8(read_request(&mut stream)).unwrap());
            write!(
                stream,
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                body.len()
            )
            .unwrap();
        }
        requests
    });

    let output = xtask(&format!("{base_url}/manifest"), &skills_root)
        .arg("--create-issue")
        .env("GITHUB_REPOSITORY", "owner/repo")
        .env("GITHUB_TOKEN", "test-token")
        .env("GITHUB_API_URL", &base_url)
        .output()
        .unwrap();

    let stdout = assert_success(&output);
    assert!(stdout.contains(&format!("Created issue: {base_url}/owner/repo/issues/1")));
    let requests = server.join().unwrap();
    assert!(requests[1].starts_with("GET /repos/owner/repo/issues?"));
    assert!(requests[2].starts_with("POST /repos/owner/repo/issues HTTP/1.1"));
    assert!(requests[2].contains("\"title\":\"sync skills: rust 1.98\""));
}

#[test]
fn reports_github_api_failures() {
    let tempdir = tempfile::tempdir().unwrap();
    let skills_root = tempdir.path().join("skills");
    write_tracked_skill(&skills_root, "1.97");
    let (base_url, server) = spawn_server(vec![
        TestResponse::ok(stable_manifest("1.98.0", None)),
        TestResponse {
            status: "500 Internal Server Error",
            body: "{}".to_owned(),
        },
    ]);

    let list_failure = xtask(&format!("{base_url}/manifest"), &skills_root)
        .arg("--create-issue")
        .env("GITHUB_REPOSITORY", "owner/repo")
        .env("GITHUB_TOKEN", "test-token")
        .env("GITHUB_API_URL", &base_url)
        .output()
        .unwrap();

    assert!(!list_failure.status.success());
    assert!(
        String::from_utf8_lossy(&list_failure.stderr).contains("failed to list open GitHub issues")
    );
    server.join().unwrap();

    let (base_url, server) = spawn_server(vec![
        TestResponse::ok(stable_manifest("1.98.0", None)),
        TestResponse::ok("[]"),
        TestResponse {
            status: "500 Internal Server Error",
            body: "{}".to_owned(),
        },
    ]);
    let create_failure = xtask(&format!("{base_url}/manifest"), &skills_root)
        .arg("--create-issue")
        .env("GITHUB_REPOSITORY", "owner/repo")
        .env("GITHUB_TOKEN", "test-token")
        .env("GITHUB_API_URL", &base_url)
        .output()
        .unwrap();

    assert!(!create_failure.status.success());
    assert!(
        String::from_utf8_lossy(&create_failure.stderr).contains("failed to create GitHub issue")
    );
    server.join().unwrap();
}

#[test]
fn requires_github_environment_to_create_an_issue() {
    let tempdir = tempfile::tempdir().unwrap();
    let skills_root = tempdir.path().join("skills");
    write_tracked_skill(&skills_root, "1.97");
    let (base_url, server) = spawn_server(vec![TestResponse::ok(stable_manifest("1.98.0", None))]);

    let missing_repository = xtask(&format!("{base_url}/manifest"), &skills_root)
        .arg("--create-issue")
        .output()
        .unwrap();

    assert!(!missing_repository.status.success());
    assert!(
        String::from_utf8_lossy(&missing_repository.stderr)
            .contains("GITHUB_REPOSITORY is required to create issues")
    );
    server.join().unwrap();

    let (base_url, server) = spawn_server(vec![TestResponse::ok(stable_manifest("1.98.0", None))]);
    let missing_token = xtask(&format!("{base_url}/manifest"), &skills_root)
        .arg("--create-issue")
        .env("GITHUB_REPOSITORY", "owner/repo")
        .output()
        .unwrap();

    assert!(!missing_token.status.success());
    assert!(
        String::from_utf8_lossy(&missing_token.stderr)
            .contains("GITHUB_TOKEN is required to create issues")
    );
    server.join().unwrap();
}

#[test]
fn rejects_invalid_github_configuration() {
    let tempdir = tempfile::tempdir().unwrap();
    let skills_root = tempdir.path().join("skills");
    write_tracked_skill(&skills_root, "1.97");
    let (base_url, server) = spawn_server(vec![TestResponse::ok(stable_manifest("1.98.0", None))]);

    let invalid_repository = xtask(&format!("{base_url}/manifest"), &skills_root)
        .arg("--create-issue")
        .env("GITHUB_REPOSITORY", "owner")
        .env("GITHUB_TOKEN", "test-token")
        .output()
        .unwrap();

    assert!(!invalid_repository.status.success());
    assert!(
        String::from_utf8_lossy(&invalid_repository.stderr)
            .contains("GitHub repository must be in `owner/repo` form")
    );
    server.join().unwrap();

    let (base_url, server) = spawn_server(vec![TestResponse::ok(stable_manifest("1.98.0", None))]);
    let invalid_api_url = xtask(&format!("{base_url}/manifest"), &skills_root)
        .arg("--create-issue")
        .env("GITHUB_REPOSITORY", "owner/repo")
        .env("GITHUB_TOKEN", "test-token")
        .env("GITHUB_API_URL", "not a URL")
        .output()
        .unwrap();

    assert!(!invalid_api_url.status.success());
    assert!(
        String::from_utf8_lossy(&invalid_api_url.stderr)
            .contains("failed to configure GitHub API base URI")
    );
    server.join().unwrap();
}

use std::str::FromStr;

use anyhow::{Context as _, Result, bail};
use octocrab::{Octocrab, params};

use super::{RustSyncState, report::build_issue_body};

#[derive(Debug)]
pub(super) struct Repository {
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

pub(super) struct GitHubClient {
    client: Octocrab,
    repository: Repository,
}

impl GitHubClient {
    pub(super) fn new(api_url: String, repository: Repository, token: String) -> Result<Self> {
        let client = Octocrab::builder()
            .personal_token(token)
            .base_uri(api_url)
            .context("failed to configure GitHub API base URI")?
            .build()
            .context("failed to build GitHub API client")?;

        Ok(Self { client, repository })
    }

    pub(super) async fn open_sync_issue(&self, state: &RustSyncState) -> Result<String> {
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

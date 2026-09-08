use std::collections::HashMap;

use anyhow::{Context as _, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct ChannelManifest {
    pub(super) date: Option<String>,
    pub(super) pkg: HashMap<String, ManifestPackage>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ManifestPackage {
    pub(super) version: String,
}

pub(super) async fn fetch_manifest(client: &Client, manifest_url: &str) -> Result<ChannelManifest> {
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

use std::{env, fs, path::PathBuf, process::Command};

use anyhow::{Context as _, Result, ensure};

use super::discovery::Document;

pub(super) fn run(documents: &[Document]) -> Result<()> {
    let temporary = tempfile::Builder::new()
        .prefix("skill-snippets-")
        .tempdir()?;
    let root = temporary.path();
    fs::create_dir(root.join("src"))?;
    fs::write(
        root.join("Cargo.toml"),
        format!(
            "[package]\nname = \"skill-snippets\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[workspace]\n\n{}",
            include_str!("dependencies.toml")
        ),
    )?;
    let mut library = String::new();
    for (index, doc) in documents.iter().enumerate() {
        let relative = PathBuf::from("skills").join(&doc.path);
        let destination = root.join(&relative);
        fs::create_dir_all(
            destination
                .parent()
                .context("snippet document needs a parent")?,
        )?;
        fs::write(destination, &doc.markdown)?;
        let include = format!("../{}", relative.to_string_lossy().replace('\\', "/"));
        library.push_str(&format!(
            "#[doc = include_str!({include:?})]\npub mod document_{index} {{}}\n"
        ));
    }
    fs::write(root.join("src/lib.rs"), library)?;

    let target = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/skill-snippets");
    let status = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .args(["test", "--doc", "--manifest-path"])
        .arg(root.join("Cargo.toml"))
        .current_dir(root)
        .env("CARGO_TARGET_DIR", target)
        .env("RUSTC_WRAPPER", "")
        .env("RUSTC_WORKSPACE_WRAPPER", "")
        .env("INSTA_UPDATE", "no")
        .env("INSTA_FORCE_PASS", "0")
        .status()
        .context("failed to run Cargo for skill snippets")?;
    ensure!(status.success(), "Rust snippet checks failed ({status})");
    Ok(())
}

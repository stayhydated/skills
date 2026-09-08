use std::{
    collections::BTreeSet,
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context as _, Result, ensure};
use walkdir::WalkDir;

use super::discovery::Document;

pub(super) fn run(skills_root: &Path, selected: &[Document]) -> Result<()> {
    let skills = selected
        .iter()
        .filter(|document| !document.snippets.is_empty())
        .map(|document| skill(&document.path))
        .collect::<Result<BTreeSet<_>>>()?;
    let output = output_root();

    for skill in skills {
        let crate_root = output.join(skill);
        let setup = load_setup(skill)?;
        prepare(skills_root, skill, &crate_root, &setup)?;
        println!(
            "Testing {} Markdown with generated crate {}",
            skill.display(),
            crate_root.display()
        );
        test(&crate_root, &output).with_context(|| {
            format!(
                "{} snippet checks failed; generated crate retained at {}",
                skill.display(),
                crate_root.display()
            )
        })?;
    }
    Ok(())
}

fn skill(path: &Path) -> Result<&Path> {
    let first = path.components().next().context("document needs a skill")?;
    ensure!(
        path.components().count() > 1,
        "snippet documents must be inside a skill directory: {}",
        path.display()
    );
    Ok(Path::new(first.as_os_str()))
}

fn output_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be inside the workspace")
        .join("target/skill-snippets")
}

fn load_setup(skill: &Path) -> Result<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/commands/check_skill_snippets/setups")
        .join(skill)
        .with_extension("toml");
    fs::read_to_string(&path).with_context(|| {
        format!(
            "missing snippet setup for {}: {}",
            skill.display(),
            path.display()
        )
    })
}

fn prepare(skills_root: &Path, skill: &Path, root: &Path, setup: &str) -> Result<()> {
    fs::create_dir_all(root.join("src"))?;
    let package = format!("{}-skill-snippets", skill.to_string_lossy());
    fs::write(
        root.join("Cargo.toml"),
        format!(
            "[package]\nname = {package:?}\nversion = \"0.0.0\"\nedition = \"2024\"\nrust-version = \"1.97\"\npublish = false\n\n[workspace]\n\n{setup}"
        ),
    )?;

    let skill_root = fs::canonicalize(skills_root.join(skill))
        .with_context(|| format!("failed to resolve skill directory {}", skill.display()))?;
    let mut library = String::new();
    for (index, entry) in WalkDir::new(&skill_root)
        .sort_by_file_name()
        .into_iter()
        .enumerate()
    {
        let entry = entry.context("failed to walk skill directory")?;
        if !entry.file_type().is_file()
            || entry
                .path()
                .extension()
                .is_none_or(|extension| extension != "md")
        {
            continue;
        }
        let markdown = entry.path();
        let markdown = markdown
            .to_str()
            .with_context(|| format!("Markdown path is not UTF-8: {}", markdown.display()))?;
        library.push_str(&format!(
            "#[doc = include_str!({markdown:?})]\npub mod document_{index} {{}}\n"
        ));
    }
    ensure!(
        !library.is_empty(),
        "no Markdown documents found for {}",
        skill.display()
    );
    fs::write(root.join("src/lib.rs"), library)?;
    Ok(())
}

fn test(root: &Path, output: &Path) -> Result<()> {
    let status = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .args(["test", "--doc", "--manifest-path"])
        .arg(root.join("Cargo.toml"))
        .current_dir(root)
        .env("CARGO_TARGET_DIR", output.join("build"))
        .env("RUSTC_WRAPPER", "")
        .env("RUSTC_WORKSPACE_WRAPPER", "")
        .env("INSTA_UPDATE", "no")
        .env("INSTA_FORCE_PASS", "0")
        .status()
        .context("failed to run Cargo for skill snippets")?;
    ensure!(status.success(), "Rust snippet checks failed ({status})");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preparation_imports_every_skill_document_into_one_program() {
        let skills = tempfile::tempdir().unwrap();
        let output = tempfile::tempdir().unwrap();
        fs::create_dir_all(skills.path().join("example/references")).unwrap();
        fs::write(skills.path().join("example/SKILL.md"), "# Skill\n").unwrap();
        fs::write(
            skills.path().join("example/references/guide.md"),
            "# Guide\n",
        )
        .unwrap();
        prepare(skills.path(), Path::new("example"), output.path(), "").unwrap();

        let library = fs::read_to_string(output.path().join("src/lib.rs")).unwrap();
        assert_eq!(library.matches("include_str!").count(), 2);
        let skill_root = fs::canonicalize(skills.path().join("example")).unwrap();
        for markdown in [
            skill_root.join("SKILL.md"),
            skill_root.join("references/guide.md"),
        ] {
            let markdown = markdown.to_str().unwrap();
            assert!(
                library.contains(&format!("include_str!({markdown:?})")),
                "generated library did not import {markdown}\n{library}"
            );
        }
        assert!(output.path().join("Cargo.toml").is_file());
    }
}

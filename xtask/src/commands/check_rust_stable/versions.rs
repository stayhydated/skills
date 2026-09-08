use std::{collections::BTreeMap, fs, path::Path, str::FromStr};

use anyhow::{Context as _, Result, bail};
use regex::Regex;
use walkdir::WalkDir;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct RustMinorVersion {
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

pub(super) fn scan_tracked_rust_versions(
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
}

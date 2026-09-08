use std::{fs, path::Path};

use anyhow::{Context as _, Result};

use super::RustSyncState;

pub(super) fn build_issue_body(state: &RustSyncState) -> String {
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

pub(super) fn print_result(state: &RustSyncState, result: &str) {
    println!("Latest Rust stable: {}", state.latest_full);
    println!(
        "Tracked Rust baseline: {}",
        state
            .tracked_minor
            .map_or_else(|| "not found".to_owned(), |version| version.to_string())
    );
    println!("{result}");
}

pub(super) fn append_step_summary(
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

#[cfg(test)]
mod tests {
    use super::*;

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
}

use std::{fs, path::PathBuf, process::Command};

fn list_snippets(source: &str, deny_ignored: bool) -> std::process::Output {
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join("example.md"), source).unwrap();
    let mut command = Command::new(env!("CARGO_BIN_EXE_xtask"));
    command
        .args(["check-skill-snippets", "--skills-root"])
        .arg(root.path())
        .arg("--list");
    if deny_ignored {
        command.arg("--deny-ignored");
    }
    command.output().unwrap()
}

#[test]
fn deny_ignored_covers_attributes_after_comments() {
    let source = "```rust (requires an external service),ignore\nmissing_crate::broken();\n```\n";
    let listed = list_snippets(source, false);
    assert!(listed.status.success());
    assert!(String::from_utf8_lossy(&listed.stdout).contains("1 ignored"));

    let denied = list_snippets(source, true);
    assert!(!denied.status.success());
    assert!(String::from_utf8_lossy(&denied.stderr).contains("1 Rust snippets are ignored"));
}

#[test]
fn generated_program_lives_in_target_and_imports_all_skill_markdown() {
    let root = tempfile::tempdir().unwrap();
    let skill = root.path().join("rust-test");
    fs::create_dir_all(&skill).unwrap();
    fs::write(
        skill.join("selected.md"),
        "```rust\nassert_eq!(2 + 2, 4);\n```\n",
    )
    .unwrap();
    fs::write(
        skill.join("also-included.md"),
        "```rust\nassert_eq!(3 + 4, 7);\n```\n",
    )
    .unwrap();

    let run = || {
        Command::new(env!("CARGO_BIN_EXE_xtask"))
            .args(["check-skill-snippets", "--skills-root"])
            .arg(root.path())
            .args(["--filter", "selected"])
            .output()
            .unwrap()
    };
    let output = run();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let generated =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/skill-snippets/rust-test");
    let library = fs::read_to_string(generated.join("src/lib.rs")).unwrap();
    assert_eq!(library.matches("include_str!").count(), 2);
    assert!(library.contains("selected.md"));
    assert!(library.contains("also-included.md"));
    assert!(generated.join("Cargo.toml").is_file());
    assert!(!skill.join("Cargo.toml").exists());

    fs::write(
        skill.join("also-included.md"),
        "```rust\nthis_function_does_not_exist();\n```\n",
    )
    .unwrap();
    let output = run();
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("generated crate retained at"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

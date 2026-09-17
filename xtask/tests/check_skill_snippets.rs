use std::{fs, path::PathBuf, process::Command, sync::Mutex};

static SNIPPET_RUNNER: Mutex<()> = Mutex::new(());

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
    let _guard = SNIPPET_RUNNER.lock().unwrap();
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
    let manifest = fs::read_to_string(generated.join("Cargo.toml")).unwrap();
    assert_eq!(manifest.matches("[[test]]").count(), 2);
    let tests = fs::read_to_string(generated.join("tests/document_0_also_included.rs")).unwrap()
        + &fs::read_to_string(generated.join("tests/document_1_selected.rs")).unwrap();
    assert!(tests.contains("2 + 2"));
    assert!(tests.contains("3 + 4"));
    assert!(generated.join("src/lib.rs").is_file());
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

#[test]
fn aggregate_snippets_share_items_run_tests_and_enforce_clippy() {
    let _guard = SNIPPET_RUNNER.lock().unwrap();
    let root = tempfile::tempdir().unwrap();
    let skill = root.path().join("rust-test");
    fs::create_dir_all(&skill).unwrap();
    let markdown = skill.join("guide.md");
    let run = || {
        Command::new(env!("CARGO_BIN_EXE_xtask"))
            .args(["check-skill-snippets", "--skills-root"])
            .arg(root.path())
            .output()
            .unwrap()
    };

    fs::write(
        &markdown,
        "```rust\nfn answer() -> u32 { 42 }\n```\n\n```rust\n#[test]\nfn uses_an_earlier_snippet() { assert_eq!(answer(), 42); }\n```\n",
    )
    .unwrap();
    let output = run();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    fs::write(
        &markdown,
        "```rust\n#[test]\nfn generated_tests_execute() { panic!(\"test body ran\"); }\n```\n",
    )
    .unwrap();
    let output = run();
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("test body ran")
            || String::from_utf8_lossy(&output.stderr).contains("test body ran")
    );

    fs::write(
        &markdown,
        "```rust,no_run\n#[test]\nfn external_fixture_is_not_run() { panic!(\"missing fixture\"); }\n```\n",
    )
    .unwrap();
    let output = run();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    fs::write(
        &markdown,
        "```rust\nfn takes_owned_text(value: &String) -> usize { value.len() }\n```\n",
    )
    .unwrap();
    let output = run();
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("clippy::ptr_arg")
            || String::from_utf8_lossy(&output.stderr).contains("clippy::ptr_arg")
    );
}

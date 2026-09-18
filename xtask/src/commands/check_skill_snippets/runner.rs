use std::{
    collections::BTreeSet,
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context as _, Result, ensure};
use syn::{Item, Stmt};

use super::discovery::{self, Document, Snippet};

struct Setup {
    rust_version: String,
    cargo: String,
}

pub(super) fn run(skills_root: &Path, selected: &[Document]) -> Result<()> {
    let skills = selected
        .iter()
        .filter(|document| !document.snippets.is_empty())
        .map(|document| skill(&document.path))
        .collect::<Result<BTreeSet<_>>>()?;
    let documents = discovery::discover(skills_root, None)?;
    let output = output_root();
    let lints = load_workspace_lints()?;

    for skill in skills {
        let crate_root = output.join(skill);
        let setup = load_setup(skill)?;
        let skill_documents = documents
            .iter()
            .filter(|document| document.path.starts_with(skill))
            .collect::<Vec<_>>();
        prepare(skill, &crate_root, &setup, &lints, &skill_documents)?;
        println!(
            "Testing {} snippets with generated crate {}",
            skill.display(),
            crate_root.display()
        );
        validate(&crate_root, &output).with_context(|| {
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

fn load_setup(skill: &Path) -> Result<Setup> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/commands/check_skill_snippets/setups")
        .join(skill)
        .with_extension("toml");
    let source = fs::read_to_string(&path).with_context(|| {
        format!(
            "missing snippet setup for {}: {}",
            skill.display(),
            path.display()
        )
    })?;
    let mut setup: toml::Table = toml::from_str(&source)
        .with_context(|| format!("invalid snippet setup {}", path.display()))?;
    let rust_version = setup
        .remove("rust-version")
        .context("snippet setup needs rust-version")?
        .as_str()
        .context("snippet setup rust-version must be a string")?
        .to_owned();
    Ok(Setup {
        rust_version,
        cargo: toml::to_string(&setup)?,
    })
}

fn load_workspace_lints() -> Result<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be inside the workspace")
        .join("Cargo.toml");
    let source = fs::read_to_string(&path)?;
    let manifest: toml::Table = toml::from_str(&source)?;
    let lints = manifest
        .get("workspace")
        .and_then(toml::Value::as_table)
        .and_then(|workspace| workspace.get("lints"))
        .and_then(toml::Value::as_table)
        .context("workspace manifest needs workspace.lints")?;
    let mut output = String::new();
    for (name, config) in lints {
        let config = config
            .as_table()
            .with_context(|| format!("workspace lint group {name} must be a table"))?;
        output.push_str(&format!("[workspace.lints.{name}]\n"));
        for (lint, setting) in config {
            output.push_str(&format!("{lint} = {setting}\n"));
        }
        output.push('\n');
    }
    Ok(output)
}

fn prepare(
    skill: &Path,
    root: &Path,
    setup: &Setup,
    lints: &str,
    documents: &[&Document],
) -> Result<()> {
    fs::create_dir_all(root.join("src"))?;
    fs::create_dir_all(root.join("tests"))?;
    let package = format!("{}-skill-snippets", skill.to_string_lossy());
    let mut targets = String::new();
    let mut doctests = String::new();

    for (index, document) in documents.iter().enumerate() {
        let ordinary = document
            .snippets
            .iter()
            .filter(|snippet| aggregate(snippet))
            .collect::<Vec<_>>();
        if !ordinary.is_empty() {
            let name = target_name(index, &document.path)?;
            let path = format!("tests/{name}.rs");
            fs::write(
                root.join(&path),
                render_document(&document.path, &ordinary)?,
            )?;
            targets.push_str(&format!("[[test]]\nname = {name:?}\npath = {path:?}\n\n"));
        }
        for snippet in document
            .snippets
            .iter()
            .filter(|snippet| !aggregate(snippet))
        {
            let markdown = format!("```{}\n{}\n```", snippet.info, snippet.code);
            doctests.push_str(&format!(
                "#[doc = {markdown:?}]\npub mod document_{index}_line_{} {{}}\n",
                snippet.line
            ));
        }
    }

    fs::write(root.join("src/lib.rs"), doctests)?;
    fs::write(
        root.join("Cargo.toml"),
        format!(
            "[package]\nname = {package:?}\nversion = \"0.0.0\"\nedition = \"2024\"\nrust-version = {:?}\npublish = false\nautotests = false\n\n[workspace]\n\n[lints]\nworkspace = true\n\n{lints}{}\n{targets}",
            setup.rust_version, setup.cargo
        ),
    )?;
    Ok(())
}

fn target_name(index: usize, path: &Path) -> Result<String> {
    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .with_context(|| {
            format!(
                "snippet document needs a UTF-8 file stem: {}",
                path.display()
            )
        })?;
    let stem = stem
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    Ok(format!("document_{index}_{stem}"))
}

fn aggregate(snippet: &Snippet) -> bool {
    !snippet.ignored
        && !snippet.attributes.iter().any(|attribute| {
            matches!(
                attribute.as_str(),
                "compile_fail"
                    | "should_panic"
                    | "edition2015"
                    | "edition2018"
                    | "edition2021"
                    | "edition2024"
                    | "standalone_crate"
            )
        })
}

fn render_document(path: &Path, snippets: &[&Snippet]) -> Result<String> {
    let mut rendered = Vec::new();
    for snippet in snippets {
        let code = unhide(&snippet.code);
        let no_run = snippet
            .attributes
            .iter()
            .any(|attribute| attribute == "no_run");
        let (attributes, mut items, statements) = if let Ok(block) =
            syn::parse_str::<syn::Block>(&format!("{{\n{code}\n}}"))
        {
            let mut items = Vec::new();
            let mut statements = Vec::new();
            for statement in block.stmts {
                match statement {
                    Stmt::Item(item) => items.push(item),
                    statement => statements.push(statement),
                }
            }
            (Vec::new(), items, statements)
        } else {
            let file: syn::File = syn::parse_str(&code)
                .with_context(|| format!("failed to parse {}:{}", path.display(), snippet.line))?;
            (file.attrs, file.items, Vec::new())
        };
        if no_run {
            for item in &mut items {
                strip_test_attributes(item);
            }
        }
        let has_main = !no_run
            && items
                .iter()
                .any(|item| matches!(item, Item::Fn(function) if function.sig.ident == "main"));
        let attributes = quote::quote!(#(#attributes)*).to_string();
        let items = quote::quote!(#(#items)*).to_string();
        let statements = (!statements.is_empty()).then(|| {
            let statements = quote::quote!(#(#statements)*).to_string();
            let test = if no_run { "" } else { "#[test]" };
            let name = if no_run { "compile" } else { "run" };
            format!("{test} fn {name}() -> impl ::std::process::Termination {{ {statements} }}")
        });
        let main = has_main.then(|| {
            "#[test] fn run_main() -> impl ::std::process::Termination { main() }".to_owned()
        });
        rendered.push((
            snippet.line,
            attributes,
            items,
            statements.unwrap_or_default(),
            main.unwrap_or_default(),
        ));
    }

    let mut nested = String::new();
    for (line, attributes, items, statements, main) in rendered.into_iter().rev() {
        nested = format!(
            "mod snippet_line_{line} {{ {attributes} #[allow(unused_imports)] use super::*; {items} {statements} {main} {nested} }}\n"
        );
    }
    let source = format!(
        "#![allow(dead_code, unused_assignments, unused_attributes, unused_imports, unused_mut, unused_variables)]\n{nested}"
    );
    let file: syn::File = syn::parse_str(&source)
        .with_context(|| format!("failed to generate {}", path.display()))?;
    Ok(prettyplease::unparse(&file))
}

fn strip_test_attributes(item: &mut Item) {
    match item {
        Item::Fn(function) => function.attrs.retain(|attribute| {
            let path = attribute.path();
            !path.is_ident("test") && !path.is_ident("should_panic") && !path.is_ident("ignore")
        }),
        Item::Mod(module) => {
            if let Some((_, items)) = &mut module.content {
                for item in items {
                    strip_test_attributes(item);
                }
            }
        },
        _ => {},
    }
}

fn unhide(code: &str) -> String {
    code.split_inclusive('\n')
        .map(|line| {
            if let Some(escaped) = line.strip_prefix("##") {
                format!("#{escaped}")
            } else if let Some(hidden) = line.strip_prefix("# ") {
                hidden.to_owned()
            } else if line == "#\n" || line == "#" {
                line.trim_start_matches('#').to_owned()
            } else {
                line.to_owned()
            }
        })
        .collect()
}

fn validate(root: &Path, output: &Path) -> Result<()> {
    cargo(
        root,
        output,
        [
            "clippy",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    cargo(root, output, ["test", "--tests", "--all-features"])?;
    cargo(root, output, ["test", "--doc", "--all-features"])?;
    Ok(())
}

fn cargo<const N: usize>(root: &Path, output: &Path, args: [&str; N]) -> Result<()> {
    let status = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .arg(args[0])
        .args(["--manifest-path"])
        .arg(root.join("Cargo.toml"))
        .args(&args[1..])
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
    fn preparation_generates_one_test_target_per_document() {
        let skills = tempfile::tempdir().unwrap();
        let output = tempfile::tempdir().unwrap();
        fs::create_dir_all(skills.path().join("example/references")).unwrap();
        fs::write(
            skills.path().join("example/SKILL.md"),
            "```rust\nassert_eq!(2 + 2, 4);\n```\n",
        )
        .unwrap();
        fs::write(
            skills.path().join("example/references/guide.md"),
            "```rust\nfn answer() -> u32 { 42 }\n```\n",
        )
        .unwrap();
        let documents = discovery::discover(skills.path(), None).unwrap();
        let documents = documents.iter().collect::<Vec<_>>();
        prepare(
            Path::new("example"),
            output.path(),
            &Setup {
                rust_version: "1.98".to_owned(),
                cargo: String::new(),
            },
            "",
            &documents,
        )
        .unwrap();

        let manifest = fs::read_to_string(output.path().join("Cargo.toml")).unwrap();
        assert_eq!(manifest.matches("[[test]]").count(), 2);
        assert!(output.path().join("tests/document_0_skill.rs").is_file());
        assert!(output.path().join("tests/document_1_guide.rs").is_file());
    }

    #[test]
    fn hidden_lines_are_compiled_and_hash_escapes_are_preserved() {
        assert_eq!(
            unhide("# let value = 42;\n##[test]\n#\n"),
            "let value = 42;\n#[test]\n\n"
        );
    }
}

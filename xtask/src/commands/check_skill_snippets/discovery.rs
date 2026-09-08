use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context as _, Result, ensure};
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use walkdir::WalkDir;

#[derive(Debug)]
pub(super) struct Snippet {
    pub(super) line: usize,
    pub(super) info: String,
    pub(super) ignored: bool,
}

pub(super) struct Document {
    pub(super) path: PathBuf,
    pub(super) snippets: Vec<Snippet>,
}

pub(super) fn discover(skills_root: &Path, filter: Option<&str>) -> Result<Vec<Document>> {
    ensure!(
        skills_root.is_dir(),
        "skills root must be a directory: {}",
        skills_root.display()
    );
    let mut documents = Vec::new();
    for entry in WalkDir::new(skills_root).sort_by_file_name() {
        let entry = entry.context("failed to walk skills directory")?;
        if !entry.file_type().is_file() || entry.path().extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        let path = entry.path().strip_prefix(skills_root)?.to_owned();
        if filter.is_some_and(|filter| !path.to_string_lossy().replace('\\', "/").contains(filter))
        {
            continue;
        }
        let source = fs::read_to_string(entry.path())
            .with_context(|| format!("failed to read {}", entry.path().display()))?;
        let snippets = extract(&source)
            .with_context(|| format!("invalid snippet in {}", entry.path().display()))?;
        if !snippets.is_empty() {
            documents.push(Document { path, snippets });
        }
    }

    Ok(documents)
}

// CommonMark parsing distinguishes real fences (including lists and quotes)
// from Rust shown inside an outer Markdown example.
fn extract(source: &str) -> Result<Vec<Snippet>> {
    let mut snippets = Vec::new();
    let mut events = Parser::new(source).into_offset_iter();
    while let Some((event, range)) = events.next() {
        let Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) = event else {
            continue;
        };
        let first = info.split([',', ' ', '\t', '(']).next().unwrap_or_default();
        if !rust_attribute(first) {
            continue;
        }
        let line = source[..range.start]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count()
            + 1;
        let (attributes, has_reason) = fence_attributes(&info)
            .with_context(|| format!("line {line}: invalid Rust fence attributes"))?;
        for attr in &attributes {
            ensure!(
                rust_attribute(attr),
                "line {line}: unsupported Rust fence attribute `{attr}`"
            );
        }
        let ignored = attributes.contains(&"ignore");
        if ignored {
            ensure!(
                has_reason,
                "line {line}: ignore requires a reason in parentheses"
            );
        }
        for (event, _) in events.by_ref() {
            if event == Event::End(TagEnd::CodeBlock) {
                break;
            }
        }
        snippets.push(Snippet {
            line,
            info: info.to_string(),
            ignored,
        });
    }
    Ok(snippets)
}

fn fence_attributes(mut info: &str) -> Result<(Vec<&str>, bool)> {
    let mut attributes = Vec::new();
    let mut has_reason = false;
    while !info.is_empty() {
        info = info.trim_start_matches([',', ' ', '\t']);
        if let Some(comment) = info.strip_prefix('(') {
            let (reason, rest) = comment
                .split_once(')')
                .context("unclosed parenthesized comment")?;
            ensure!(
                !reason.contains('('),
                "nested parenthesized comments are unsupported"
            );
            has_reason |= !reason.trim().is_empty();
            info = rest;
        } else {
            let end = info.find([',', ' ', '\t', '(']).unwrap_or(info.len());
            if end > 0 {
                attributes.push(&info[..end]);
            }
            info = &info[end..];
        }
    }
    Ok((attributes, has_reason))
}

fn rust_attribute(attribute: &str) -> bool {
    matches!(
        attribute,
        "rust"
            | "no_run"
            | "compile_fail"
            | "should_panic"
            | "ignore"
            | "test_harness"
            | "edition2015"
            | "edition2018"
            | "edition2021"
            | "edition2024"
            | "standalone_crate"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovery_preserves_source_lines_and_excludes_nested_examples() {
        let source = "heading\n```sh\necho hello\n```\n````markdown\n```rust\nnot real Rust\n```\n````\n~~~rust,no_run\nlet value = 42;\n~~~\n```rust,test_harness\n#[test]\nfn works() { assert!(true); }\n```\n";
        let snippets = extract(source).unwrap();
        assert_eq!(
            snippets
                .iter()
                .map(|snippet| snippet.line)
                .collect::<Vec<_>>(),
            [10, 13]
        );
    }

    #[test]
    fn discovers_fences_in_lists_and_quotes_without_executing_inline_code() {
        let source = "`let x = 1;`\n\n> ```rust\n> assert_eq!(1, 1);\n> ```\n\n- Example:\n\n  ~~~rust\n  assert!(true);\n  ~~~\n";
        let snippets = extract(source).unwrap();
        assert_eq!(
            snippets
                .iter()
                .map(|snippet| snippet.line)
                .collect::<Vec<_>>(),
            [3, 9]
        );
    }

    #[test]
    fn invalid_fences_cannot_silently_drop_checks() {
        for source in [
            "```rust,no_rnu\nloop {}\n```\n",
            "```rust,ignore\n?\n```\n",
            "```rust (reason),no_rnu\nloop {}\n```\n",
            "```rust ( ),ignore\n?\n```\n",
            "```rust (unclosed\n?\n```\n",
            "```rust (nested (comment)),ignore\n?\n```\n",
        ] {
            assert!(extract(source).is_err(), "{source}");
        }
        let snippets = extract("```rust,ignore (requires a proc-macro crate)\n?\n```\n").unwrap();
        assert!(snippets[0].ignored);
    }

    #[test]
    fn comments_preserve_attributes_and_ignore_accounting() {
        for header in [
            "rust,ignore (external service)",
            "rust (external service),ignore",
            "rust (external service) ignore (outline)",
        ] {
            let snippets = extract(&format!("```{header}\n?\n```\n")).unwrap();
            assert!(snippets[0].ignored, "{header}");
        }
        let snippets =
            extract("```rust (ignore is only commentary),no_run\nloop {}\n```\n").unwrap();
        assert!(!snippets[0].ignored);
    }
}

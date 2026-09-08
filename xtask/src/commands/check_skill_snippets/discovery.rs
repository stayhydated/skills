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
    pub(super) markdown: String,
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
        let (markdown, snippets) = extract(&source)
            .with_context(|| format!("invalid snippet in {}", entry.path().display()))?;
        if !snippets.is_empty() {
            documents.push(Document {
                path,
                markdown,
                snippets,
            });
        }
    }

    Ok(documents)
}

// CommonMark parsing distinguishes real fences (including lists and quotes)
// from Rust shown inside an outer Markdown example. Padding preserves locations.
fn extract(source: &str) -> Result<(String, Vec<Snippet>)> {
    let mut markdown = String::new();
    let mut snippets = Vec::new();
    let mut events = Parser::new(source).into_offset_iter();
    while let Some((event, range)) = events.next() {
        let Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) = event else {
            continue;
        };
        let attributes: Vec<_> = info
            .split_once('(')
            .map_or(info.as_ref(), |(attrs, _)| attrs)
            .split([',', ' ', '\t'])
            .filter(|part| !part.is_empty())
            .collect();
        if !attributes
            .first()
            .is_some_and(|first| rust_attribute(first))
        {
            continue;
        }
        let line = source[..range.start]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count()
            + 1;
        for attr in &attributes {
            ensure!(
                rust_attribute(attr),
                "line {line}: unsupported Rust fence attribute `{attr}`"
            );
        }
        let ignored = attributes.contains(&"ignore");
        if ignored {
            ensure!(
                info.split_once('(')
                    .and_then(|(_, reason)| reason.strip_suffix(')'))
                    .is_some_and(|reason| !reason.trim().is_empty()),
                "line {line}: ignore requires a reason in parentheses"
            );
        }
        let mut code = String::new();
        for (event, _) in events.by_ref() {
            match event {
                Event::Text(text) => code.push_str(&text),
                Event::End(TagEnd::CodeBlock) => break,
                _ => {},
            }
        }
        // A longer fence keeps literal backticks inside the Rust snippet intact.
        let fence_len = code
            .split(|ch| ch != '`')
            .map(str::len)
            .max()
            .unwrap_or(0)
            .max(2)
            + 1;
        let fence = "`".repeat(fence_len);
        while markdown.lines().count() < line - 1 {
            markdown.push('\n');
        }
        markdown.push_str(&format!("{fence}{info}\n{code}"));
        if !markdown.ends_with('\n') {
            markdown.push('\n');
        }
        markdown.push_str(&format!("{fence}\n"));
        snippets.push(Snippet {
            line,
            info: info.to_string(),
            ignored,
        });
    }
    Ok((markdown, snippets))
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
        let (markdown, snippets) = extract(source).unwrap();
        assert_eq!(
            snippets
                .iter()
                .map(|snippet| snippet.line)
                .collect::<Vec<_>>(),
            [10, 13]
        );
        assert_eq!(markdown.lines().count(), source.lines().count());
        assert!(!markdown.contains("not real Rust"));
        assert_eq!(markdown.lines().nth(10), Some("let value = 42;"));
        assert_eq!(markdown.lines().nth(13), Some("#[test]"));
    }

    #[test]
    fn discovers_fences_in_lists_and_quotes_without_executing_inline_code() {
        let source = "`let x = 1;`\n\n> ```rust\n> assert_eq!(1, 1);\n> ```\n\n- Example:\n\n  ~~~rust\n  assert!(true);\n  ~~~\n";
        let (markdown, snippets) = extract(source).unwrap();
        assert_eq!(
            snippets
                .iter()
                .map(|snippet| snippet.line)
                .collect::<Vec<_>>(),
            [3, 9]
        );
        assert_eq!(markdown.lines().nth(3), Some("assert_eq!(1, 1);"));
        assert_eq!(markdown.lines().nth(9), Some("assert!(true);"));
    }

    #[test]
    fn invalid_fences_cannot_silently_drop_checks() {
        for source in ["```rust,no_rnu\nloop {}\n```\n", "```rust,ignore\n?\n```\n"] {
            assert!(extract(source).is_err(), "{source}");
        }
        let (_, snippets) =
            extract("```rust,ignore (requires a proc-macro crate)\n?\n```\n").unwrap();
        assert!(snippets[0].ignored);
    }
}

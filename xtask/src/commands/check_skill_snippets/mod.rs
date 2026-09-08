use std::path::PathBuf;

use anyhow::{Result, ensure};

mod discovery;
mod runner;

#[derive(clap::Args, Debug)]
pub(crate) struct Args {
    #[arg(long, default_value = "skills")]
    skills_root: PathBuf,
    /// Select Markdown files whose relative paths contain this text.
    #[arg(long)]
    filter: Option<String>,
    /// List discovered Rust snippets without compiling or running them.
    #[arg(long)]
    list: bool,
    /// Fail if any snippet is explicitly ignored.
    #[arg(long)]
    deny_ignored: bool,
}

pub(crate) fn run(args: &Args) -> Result<()> {
    let documents = discovery::discover(&args.skills_root, args.filter.as_deref())?;

    let count: usize = documents.iter().map(|doc| doc.snippets.len()).sum();
    let ignored = documents
        .iter()
        .flat_map(|doc| &doc.snippets)
        .filter(|snippet| snippet.ignored)
        .count();
    ensure!(
        count > 0,
        "no Rust snippets matched in {}",
        args.skills_root.display()
    );
    for doc in &documents {
        for snippet in &doc.snippets {
            if args.list || snippet.ignored {
                println!(
                    "{}:{}: {}",
                    args.skills_root.join(&doc.path).display(),
                    snippet.line,
                    snippet.info
                );
            }
        }
    }
    println!(
        "Discovered {count} Rust snippets in {} files ({ignored} ignored). Other fence languages are outside this check.",
        documents.len()
    );
    ensure!(
        !args.deny_ignored || ignored == 0,
        "{ignored} Rust snippets are ignored"
    );
    if args.list {
        return Ok(());
    }
    ensure!(count > ignored, "all discovered Rust snippets are ignored");

    runner::run(&documents)
}

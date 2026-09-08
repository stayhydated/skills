use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::{check_rust_stable, check_skill_snippets};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    CheckRustStable(check_rust_stable::Args),
    /// Compile and run Rust code fences discovered in skill Markdown files.
    CheckSkillSnippets(check_skill_snippets::Args),
}

impl Cli {
    pub(crate) async fn run(self) -> Result<()> {
        match self.command {
            Command::CheckRustStable(args) => check_rust_stable::run(&args).await,
            Command::CheckSkillSnippets(args) => check_skill_snippets::run(&args),
        }
    }
}

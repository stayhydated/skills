use anyhow::Result;
use clap::Parser as _;

mod cli;
mod commands;

#[tokio::main]
async fn main() -> Result<()> {
    cli::Cli::parse().run().await
}

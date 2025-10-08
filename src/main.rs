mod app;
mod event;
mod name;
mod related;

use crate::{app::App, name::ArticleName, related::RelatedApp};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List other articles that reference a given article
    Related { name: ArticleName },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut term = ratatui::init();

    let res = match cli.command {
        Command::Related { name } => RelatedApp::new(name).run(&mut term).await,
    };

    ratatui::restore();
    res
}

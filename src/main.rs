mod app;
mod crawl;
mod database;
mod dirs;
mod load;
mod name;
mod related;
mod url;

use crate::{
    app::App, crawl::Crawler, database::Database, dirs::Dirs, name::ArticleName,
    related::RelatedApp,
};
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
    let crawler = Crawler::new();

    let dirs = Dirs::new().await?;
    let database = Database::new(&dirs).await?;

    let mut term = ratatui::init();

    let res = match cli.command {
        Command::Related { name } => {
            RelatedApp::new(crawler, database, name)
                .run(&mut term)
                .await
        }
    };

    ratatui::restore();
    res
}

mod crawl;
mod name;
mod url;

use self::{
    crawl::Crawler,
    name::{ArticleName, GroupName},
};
use anyhow::Result;
use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List other articles that reference a given article
    Related {
        #[arg()]
        article: ArticleName,

        /// Include only articles in a group, e.g. "Literature"
        #[arg(short, long)]
        group: Option<GroupName>,

        /// Load results and filter them in interactive viewer
        #[arg(short, long)]
        interactive: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    let crawler = Crawler::new();

    match cli.command {
        Command::Related {
            article,
            group,
            interactive,
        } => {
            let related = crawler.related(&article, group.as_ref()).await?;
            dbg!(related.len());
        }
    }

    Ok(())
}

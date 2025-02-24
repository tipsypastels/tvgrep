mod crawl;
mod files;
mod list;
mod name;
mod print;

use self::{
    crawl::Crawler,
    files::{History, Verdict},
    name::{ArticleName, GroupName},
    print::Printer,
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
    /// Add an article and verdict to history
    Remember {
        #[arg()]
        article: ArticleName,

        #[arg()]
        verdict: Verdict,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer().without_time())
        .with(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    let crawler = Crawler::new();
    let mut history = History::new().await?;

    match cli.command {
        Command::Related {
            article,
            group,
            interactive,
        } => {
            let related = crawler.related(&article, group.as_ref()).await?;

            if !interactive {
                Printer::new(related.iter().filter(|a| !history.has(a)))
                    .unfiltered_len(related.len())
                    .print();

                return Ok(());
            }
        }
        Command::Remember { article, verdict } => {
            let prev_verdict = history.insert(article.clone(), verdict);
            if let Some(prev_verdict) = prev_verdict {
                if prev_verdict == verdict {
                    println!("Article {article} already has verdict {verdict}, nothing to do.");
                    return Ok(());
                } else {
                    println!("Article {article} verdict changed to {verdict}.");
                }
            } else {
                println!("Article {article} verdict set to {verdict}.");
            }
            history.flush().await?;
        }
    }

    Ok(())
}

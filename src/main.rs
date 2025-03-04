mod crawl;
mod data;
mod files;
mod list;
mod name;
mod print;
mod queue;
mod term;

use self::{
    crawl::Crawler,
    files::{History, Verdict},
    name::{ArticleName, GroupName},
    print::Printer,
};
use anyhow::Result;
use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use std::ops::ControlFlow;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(long)]
    dry_run: bool,
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
    let history = History::new(cli.dry_run).await?;

    match cli.command {
        Command::Related {
            article,
            group,
            interactive,
        } => {
            let related = crawler.related(&article, group.as_ref()).await?;
            let iter = related.iter().filter(|a| !history.has(a));
            let trope_query = crawl::trope::Stub;

            // TODO: Use trope_query here in bulk.
            if !interactive {
                Printer::new(iter).unfiltered_len(related.len()).print();
                return Ok(());
            }

            queue::make(
                iter,
                async |article| crawler.article(article, &trope_query).await,
                async |article, data| {
                    println!("{data}");
                    match term::prompt(&["Yes", "No", "Skip", "Quit"]).await? {
                        "Yes" => {
                            history.insert(article.clone(), Verdict::Yes);
                        }
                        "No" => {
                            history.insert(article.clone(), Verdict::No);
                        }
                        "Quit" => {
                            tracing::debug!("quitting");
                            return Ok(ControlFlow::Break(()));
                        }
                        _ => {}
                    }
                    Ok(ControlFlow::Continue(()))
                },
            )
            .await?;

            history.flush().await?;
        }
        Command::Remember { article, verdict } => {
            let article_link = article.display_link();
            match history.insert(article.clone(), verdict) {
                Some(prev) if prev == verdict => {
                    println!(
                        "Article {article_link} already has verdict {verdict}, nothing to do."
                    );
                }
                Some(_) => {
                    println!("Article {article_link} verdict changed to {verdict}.")
                }
                None => {
                    println!("Article {article_link} verdict set to {verdict}.");
                }
            }
            history.flush().await?;
        }
    }

    Ok(())
}

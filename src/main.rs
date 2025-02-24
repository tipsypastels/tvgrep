mod crawl;
mod files;
mod list;
mod name;
mod print;
mod queue;

use self::{
    crawl::{ArticleCrawledData, Crawler},
    files::{History, Verdict},
    name::{ArticleName, GroupName},
    print::Printer,
};
use anyhow::Result;
use clap::{Parser, Subcommand};
use dialoguer::Select;
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
    Test {
        #[arg()]
        article: ArticleName,
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
            let iter = related.iter().filter(|a| !history.has(a));

            if !interactive {
                Printer::new(iter).unfiltered_len(related.len()).print();

                return Ok(());
            }

            queue::start(
                iter,
                async |_, data: ArticleCrawledData| {
                    println!("{}\n{}", data.title, data.summary);
                    let choice = tokio::task::spawn_blocking(|| {
                        let choices = ["Yes", "No", "Skip", "Quit"];
                        let choice = Select::new()
                            .with_prompt("Choose")
                            .items(&choices)
                            .default(0)
                            .interact()?;

                        anyhow::Ok(choice)
                    })
                    .await??;

                    tracing::info!("chose {choice}");
                    Ok(())
                },
                async |article| crawler.article(article).await,
            )
            .await?;
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
        Command::Test { article } => {
            let article_data = crawler.article(&article).await?;
            dbg!(article_data);
        }
    }

    Ok(())
}

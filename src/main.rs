mod crawl;
mod data;
mod files;
mod list;
mod name;
mod print;
mod progress;
mod queue;
mod term;

use self::{
    crawl::Crawler,
    data::{ArticleData, TropeDataSingle},
    files::{History, Verdict},
    name::{ArticleName, GroupName},
    progress::Progress,
};
use anyhow::Result;
use clap::{Parser, Subcommand};
use futures::{StreamExt, stream};
use std::ops::ControlFlow;

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
            let trope_query = crawl::trope::Single(&article);

            if !interactive {
                print::print_async(
                    Some(related.len()),
                    stream::iter(iter).then(|article| {
                        let crawler = crawler.clone();
                        async move {
                            let data = crawler.article(article, &trope_query).await;
                            print::ArticleAndTropeDesc::new(article, data)
                        }
                    }),
                )
                .await;
                return Ok(());
            }

            queue::make(
                iter,
                async |article| crawler.article(article, &trope_query).await,
                async |article: &ArticleName,
                       data: ArticleData<TropeDataSingle>,
                       progress: Progress| {
                    println!("{}", data.display_with_progress(progress));
                    match term::verdict_prompt().await? {
                        term::VerdictPrompt::Yes => {
                            history.insert(article.clone(), Verdict::Yes);
                        }
                        term::VerdictPrompt::No => {
                            history.insert(article.clone(), Verdict::No);
                        }
                        term::VerdictPrompt::Quit => {
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

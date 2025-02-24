use crate::{
    crawl::Crawler,
    data::ArticleData,
    files::{History, Verdict},
    name::ArticleName,
    queue,
};
use anyhow::{Error, Result, bail};
use dialoguer::Select;
use std::ops::ControlFlow;
use tokio::task::spawn_blocking;

pub async fn verdicts<'a, I>(iter: I, crawler: &'a Crawler, history: &'a History) -> Result<()>
where
    I: Iterator<Item = &'a ArticleName>,
{
    queue::start(
        iter,
        async |article| crawler.article(article).await,
        async |article, data: ArticleData| {
            println!("{}", data);
            match get_choice().await? {
                Choice::Yes => {
                    history.insert(article.clone(), Verdict::Yes);
                }
                Choice::No => {
                    history.insert(article.clone(), Verdict::No);
                }
                Choice::Skip => {
                    // do nothing
                }
                Choice::Quit => {
                    tracing::debug!("quitting");
                    return Ok(ControlFlow::Break(()));
                }
            }
            Ok(ControlFlow::Continue(()))
        },
    )
    .await
}

async fn get_choice() -> Result<Choice> {
    spawn_blocking(|| {
        Select::new()
            .with_prompt("Choose Action")
            .items(&Choice::STRS)
            .default(Choice::Yes as _)
            .interact()
    })
    .await??
    .try_into()
}

#[derive(Copy, Clone)]
enum Choice {
    Yes,
    No,
    Skip,
    Quit,
}

impl Choice {
    const STRS: [&'static str; 4] = [
        Self::Yes.as_str(),
        Self::No.as_str(),
        Self::Skip.as_str(),
        Self::Quit.as_str(),
    ];

    const fn as_str(&self) -> &'static str {
        match self {
            Self::Yes => "Yes",
            Self::No => "No",
            Self::Skip => "Skip",
            Self::Quit => "Quit",
        }
    }
}

impl TryFrom<usize> for Choice {
    type Error = Error;

    fn try_from(n: usize) -> Result<Self> {
        Ok(match n {
            0 => Self::Yes,
            1 => Self::No,
            2 => Self::Skip,
            3 => Self::Quit,
            _ => bail!("invalid choice {n}"),
        })
    }
}

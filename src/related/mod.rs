mod crawl;
mod handle;
mod list;
mod render;

use self::{
    crawl::RelatedCrawl,
    list::{RelatedArticleEntry, RelatedArticleList},
    render::{RelatedModal, RelatedRenderer},
};
use crate::{
    app::{App, RenderInfo, Tx},
    crawl::Crawler,
    database::{Database, Verdict},
    name::{ArticleName, GroupName},
};
use anyhow::Result;
use crossterm::event::Event;
use futures::StreamExt;
use ratatui::prelude::*;
use std::{collections::HashMap, str::FromStr};

pub struct RelatedApp {
    article_name: ArticleName,
    list: RelatedArticleList,
    modal: Option<RelatedModal>,
    crawler: Crawler,
    database: Database,
}

#[must_use]
pub enum RelatedMessage {
    LoadRelated {
        group_name: Option<GroupName>,
        page: u8,
    },
    SetVerdict {
        article_name: ArticleName,
        verdict: Option<Verdict>,
    },
}

pub enum RelatedMessageOkOutput {
    Load { entries: Vec<RelatedArticleEntry> },
    Void,
}

#[derive(Clone)]
pub struct RelatedMessageContext {
    article_name: ArticleName,
    crawler: Crawler,
    database: Database,
}

impl RelatedApp {
    pub fn new(crawler: Crawler, database: Database, article_name: ArticleName) -> Self {
        Self {
            article_name: article_name.clone(),
            list: RelatedArticleList::new(),
            modal: None,
            crawler,
            database,
        }
    }
}

impl App for RelatedApp {
    type Messenger = fn(RelatedMessage, RelatedMessageContext) -> Result<RelatedMessageOkOutput>;

    fn on_start(&mut self, mut tx: Tx<Self>) {
        tx.send(self.list.load());
    }

    fn tick(&mut self, _tx: Tx<Self>) -> Result<()> {
        Ok(())
    }

    fn render(&mut self, info: RenderInfo, area: Rect, buf: &mut Buffer) {
        render::main(
            &mut RelatedRenderer {
                article_name: &self.article_name,
                list: &mut self.list,
                modal: self.modal.as_mut(),
                info,
            },
            area,
            buf,
        );
    }

    fn handle(&mut self, event: Event, tx: Tx<Self>, quit: &mut bool) -> Result<()> {
        self.handle_impl(event, tx, quit)
    }

    async fn on_message(
        message: RelatedMessage,
        context: RelatedMessageContext,
    ) -> Result<RelatedMessageOkOutput> {
        let RelatedMessageContext {
            article_name,
            crawler,
            database,
        } = context;
        match message {
            RelatedMessage::LoadRelated { group_name, page } => {
                let article_names = crawler
                    .crawl(RelatedCrawl {
                        article_name,
                        group_name,
                        page,
                    })
                    .await?;

                let mut verdicts = database.get_verdicts();
                let verdicts = {
                    let mut out = HashMap::new();
                    while let Some(verdict_entry) = verdicts.next().await {
                        let verdict_entry = verdict_entry?;
                        let article_name = ArticleName::from_str(&verdict_entry.name)?;
                        out.insert(article_name, verdict_entry.verdict);
                    }
                    out
                };

                let entries = article_names
                    .into_iter()
                    .map(|article_name| {
                        let verdict = verdicts.get(&article_name).copied();
                        RelatedArticleEntry {
                            article_name,
                            verdict,
                        }
                    })
                    .collect();

                Ok(RelatedMessageOkOutput::Load { entries })
            }
            RelatedMessage::SetVerdict {
                article_name,
                verdict: Some(verdict),
            } => {
                database.set_verdict(article_name, verdict).await?;
                Ok(RelatedMessageOkOutput::Void)
            }
            RelatedMessage::SetVerdict {
                article_name,
                verdict: None,
            } => {
                database.unset_verdict(article_name).await?;
                Ok(RelatedMessageOkOutput::Void)
            }
        }
    }

    fn apply_message(&mut self, output: Result<RelatedMessageOkOutput>) -> Result<()> {
        match output? {
            RelatedMessageOkOutput::Load { entries } => {
                self.list.loaded(entries);
                Ok(())
            }
            RelatedMessageOkOutput::Void => Ok(()),
        }
    }

    fn new_message_context(&self) -> RelatedMessageContext {
        RelatedMessageContext {
            article_name: self.article_name.clone(),
            crawler: self.crawler.clone(),
            database: self.database.clone(),
        }
    }
}

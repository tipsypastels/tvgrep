mod crawl;
mod render;

use self::crawl::RelatedCrawl;
use crate::{
    app::App,
    crawl::Crawler,
    database::{Database, Verdict},
    load::{Load, Loader},
    name::{ArticleName, GroupName},
    render::list::ListStateExt,
    task::TaskPool,
};
use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use futures::StreamExt;
use ratatui::{prelude::*, widgets::ListState};
use std::{collections::HashMap, str::FromStr};

pub struct RelatedApp {
    orig_article_name: ArticleName,
    list_state: ListState,
    list_entries: Vec<RelatedEntry>,
    list_loader: Loader<RelatedLoad>,
    list_loader_finished: bool,
    list_filter_group: Option<GroupName>,
    modal: Option<RelatedModal>,
    task_pool: TaskPool<Database, RelatedTask>,
}

impl RelatedApp {
    pub fn new(crawler: Crawler, database: Database, orig_article_name: ArticleName) -> Self {
        Self {
            orig_article_name: orig_article_name.clone(),
            list_state: ListState::default(),
            list_entries: Vec::new(),
            list_loader: Loader::new(RelatedLoad::new(
                crawler,
                database.clone(),
                orig_article_name,
            )),
            list_loader_finished: false,
            list_filter_group: None,
            modal: None,
            // TODO: Wait for this to finish before quitting.
            task_pool: TaskPool::new(database, |database, task| async move {
                match task {
                    RelatedTask::SetVerdict { name, verdict } => {
                        let _ = database.set_verdict(name, verdict).await;
                    }
                    RelatedTask::UnsetVerdict { name } => {
                        let _ = database.unset_verdict(name).await;
                    }
                }
            }),
        }
    }
}

impl App for RelatedApp {
    async fn tick(&mut self) -> Result<()> {
        if self.list_entries.is_empty()
            && !self.list_loader.is_loading()
            && !self.list_loader_finished
        {
            self.list_loader.start_loading();
        }

        if let Some(more_related_articles) = self.list_loader.read() {
            let more_related_articles = more_related_articles?;

            if more_related_articles.is_empty() {
                self.list_loader_finished = true;
            } else {
                self.list_entries.extend(more_related_articles);
            }
        }

        Ok(())
    }

    async fn handle(&mut self, event: Event, quit: &mut bool) -> Result<()> {
        let Some(code) = event.as_key_press_event().map(|e| e.code) else {
            return Ok(());
        };
        match &self.modal {
            Some(RelatedModal::SetVerdict { .. }) => self.handle_set_verdict(code),
            Some(RelatedModal::SetGroup(_)) => self.handle_set_group(code),
            None => self.handle_main(code, quit),
        }
        Ok(())
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        render::main(
            &mut RelatedRenderer {
                orig_article_name: &self.orig_article_name,
                list_state: &mut self.list_state,
                list_entries: &self.list_entries,
                list_loading: self.list_loader.is_loading(),
                modal: self.modal.as_mut(),
            },
            area,
            buf,
        );
    }
}

impl RelatedApp {
    fn handle_set_verdict(&mut self, code: KeyCode) {
        let (name, list_state) = match self.modal.as_mut() {
            Some(RelatedModal::SetVerdict { name, list_state }) => (name, list_state),
            _ => unreachable!(),
        };
        let mut set_verdict = |verdict: Option<Verdict>| {
            let task = match verdict {
                Some(verdict) => RelatedTask::SetVerdict {
                    name: name.clone(),
                    verdict,
                },
                None => RelatedTask::UnsetVerdict { name: name.clone() },
            };

            self.task_pool.send(task);

            if let Some(entry) = self
                .list_state
                .selected()
                .and_then(|i| self.list_entries.get_mut(i))
            {
                entry.verdict = verdict;
            }
        };
        match code {
            KeyCode::Up => {
                list_state.select_prev_or_last();
            }
            KeyCode::Down => {
                list_state.select_next_or_first(Verdict::VARIANT_COUNT + 1);
            }
            KeyCode::Esc => {
                self.modal = None;
            }
            KeyCode::Enter => {
                let Some(selected) = list_state.selected() else {
                    return;
                };

                set_verdict(
                    Verdict::variants()
                        .enumerate()
                        .find(|(i, _)| *i == selected)
                        .map(|(_, v)| v),
                );

                self.modal = None;
            }
            KeyCode::Char('1') => {
                set_verdict(Some(Verdict::Yes));
                self.modal = None;
            }
            KeyCode::Char('2') => {
                set_verdict(Some(Verdict::No));
                self.modal = None;
            }
            KeyCode::Char('3') => {
                set_verdict(Some(Verdict::Ignore));
                self.modal = None;
            }
            KeyCode::Char('4') => {
                set_verdict(None);
                self.modal = None;
            }
            _ => {}
        }
    }

    fn handle_set_group(&mut self, code: KeyCode) {
        let buffer = match self.modal.as_mut() {
            Some(RelatedModal::SetGroup(buffer)) => buffer,
            _ => unreachable!(),
        };
        match code {
            KeyCode::Backspace => {
                buffer.pop();
            }
            KeyCode::Char(char) => {
                buffer.push(char);
            }
            KeyCode::Esc => {
                self.modal = None;
            }
            KeyCode::Enter => {
                // TODO
                // self.list_entries = Vec::new();
                // self.list_loader_finished = false;
                self.list_filter_group = GroupName::from_str(buffer).ok();
                self.modal = None;
            }
            _ => {}
        }
    }

    fn handle_main(&mut self, code: KeyCode, quit: &mut bool) {
        match code {
            KeyCode::Up => {
                self.list_state.select_prev_or_last();
            }
            KeyCode::Down => {
                self.list_state
                    .select_next_or_first(self.list_entries.len());
            }
            KeyCode::Char('f') => {
                self.modal = Some(RelatedModal::SetGroup(
                    self.list_filter_group
                        .as_ref()
                        .map(|g| g.to_string())
                        .unwrap_or_default(),
                ));
            }
            KeyCode::Char('w') => {
                let Some(article) = self.selected_entry() else {
                    return;
                };
                self.modal = Some(RelatedModal::SetVerdict {
                    name: article.name.clone(),
                    list_state: ListState::default(),
                });
            }
            KeyCode::Char('q') => {
                *quit = true;
            }
            _ => {}
        }
    }

    fn selected_entry(&self) -> Option<&RelatedEntry> {
        self.list_state
            .selected()
            .and_then(|i| self.list_entries.get(i))
    }
}

enum RelatedModal {
    SetVerdict {
        name: ArticleName,
        list_state: ListState,
    },
    SetGroup(String),
}

struct RelatedEntry {
    name: ArticleName,
    verdict: Option<Verdict>,
}

enum RelatedTask {
    SetVerdict { name: ArticleName, verdict: Verdict },
    UnsetVerdict { name: ArticleName },
}

struct RelatedLoad {
    crawler: Crawler,
    database: Database,
    orig_article_name: ArticleName,
    page: u8,
}

impl RelatedLoad {
    fn new(crawler: Crawler, database: Database, orig_article_name: ArticleName) -> Self {
        Self {
            crawler,
            database,
            orig_article_name,
            page: 1,
        }
    }
}

impl Load for RelatedLoad {
    type Output = Result<Vec<RelatedEntry>>;

    async fn load(&mut self) -> Self::Output {
        let crawl = RelatedCrawl::new(self.orig_article_name.clone(), self.page);
        let names = self.crawler.crawl(crawl).await?;

        let mut verdicts = self.database.get_verdicts();
        let verdicts = {
            let mut out = HashMap::new();
            while let Some(verdict_entry) = verdicts.next().await {
                let verdict_entry = verdict_entry?;
                let name = ArticleName::from_str(&verdict_entry.name)?;
                out.insert(name, verdict_entry.verdict);
            }
            out
        };

        let out = names
            .into_iter()
            .map(|name| {
                let verdict = verdicts.get(&name).copied();
                RelatedEntry { name, verdict }
            })
            .collect();

        self.page += 1;
        Ok(out)
    }
}

struct RelatedRenderer<'a> {
    orig_article_name: &'a ArticleName,
    list_state: &'a mut ListState,
    list_entries: &'a [RelatedEntry],
    list_loading: bool,
    modal: Option<&'a mut RelatedModal>,
}

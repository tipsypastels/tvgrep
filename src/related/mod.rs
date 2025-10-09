mod crawl;
mod loader;
mod render;

use self::loader::RelatedLoader;
use crate::{
    app::App,
    crawl::Crawler,
    database::{Database, Verdict},
    name::{ArticleName, GroupName},
    render::list::ListStateExt,
    task::Task,
};
use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use ratatui::{prelude::*, widgets::ListState};
use std::str::FromStr;

pub struct RelatedApp {
    orig_article_name: ArticleName,
    list_state: ListState,
    list_loader: RelatedLoader,
    modal: Option<RelatedModal>,
    tasks: Task<RelatedTask, Database, ()>,
}

impl RelatedApp {
    pub fn new(crawler: Crawler, database: Database, orig_article_name: ArticleName) -> Self {
        Self {
            orig_article_name: orig_article_name.clone(),
            list_state: ListState::default(),
            list_loader: RelatedLoader::new(crawler, database.clone(), orig_article_name),
            modal: None,
            tasks: Task::new(database, |task, database| async move {
                // TODO: How to handle errors?
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
    async fn on_start(&mut self) -> Result<()> {
        self.list_loader.on_start();
        Ok(())
    }

    async fn on_quit(&mut self) -> Result<()> {
        self.tasks.close().await;
        Ok(())
    }

    async fn tick(&mut self) -> Result<()> {
        self.list_loader.tick()?;
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
                list_entries: &self.list_loader.entries(),
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

            self.tasks.run(task);

            if let Some(entry) = self
                .list_state
                .selected()
                .and_then(|i| self.list_loader.entries_mut().get_mut(i))
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
                self.list_loader
                    .set_filter_group(GroupName::from_str(buffer).ok());
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
                    .select_next_or_first(self.list_loader.entries().len());
            }
            KeyCode::Char('f') => {
                self.modal = Some(RelatedModal::SetGroup(
                    self.list_loader
                        .filter_group()
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
            .and_then(|i| self.list_loader.entries().get(i))
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

struct RelatedRenderer<'a> {
    orig_article_name: &'a ArticleName,
    list_state: &'a mut ListState,
    list_entries: &'a [RelatedEntry],
    modal: Option<&'a mut RelatedModal>,
}

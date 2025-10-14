use super::{RelatedApp, RelatedMessage, list::VerdictFilter, render::RelatedModal};
use crate::{app::Tx, database::Verdict, name::GroupName, render::list::ListStateExt};
use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use ratatui::widgets::ListState;
use std::str::FromStr;

impl RelatedApp {
    pub fn handle_impl(&mut self, event: Event, tx: Tx<Self>, quit: &mut bool) -> Result<()> {
        let Some(code) = event.as_key_press_event().map(|e| e.code) else {
            return Ok(());
        };
        match &self.modal {
            Some(RelatedModal::SetVerdict { .. }) => self.handle_set_verdict(code, tx),
            Some(RelatedModal::FilterVerdict { .. }) => self.handle_filter_verdict(code),
            Some(RelatedModal::FilterGroup { .. }) => self.handle_filter_group(code, tx),
            None => self.handle_main(code, tx, quit),
        }
        Ok(())
    }

    fn handle_set_verdict(&mut self, code: KeyCode, mut tx: Tx<Self>) {
        let (article_name, list_state) = match self.modal.as_mut() {
            Some(RelatedModal::SetVerdict {
                article_name,
                list_state,
            }) => (article_name, list_state),
            _ => unreachable!(),
        };
        let mut set_verdict = |verdict: Option<Verdict>| {
            tx.send(RelatedMessage::SetVerdict {
                article_name: article_name.clone(),
                verdict,
            });
            if let Some(entry) = self.list.selected_mut() {
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

    fn handle_filter_verdict(&mut self, code: KeyCode) {
        let list_state = match self.modal.as_mut() {
            Some(RelatedModal::FilterVerdict { list_state }) => list_state,
            _ => unreachable!(),
        };
        match code {
            KeyCode::Up => {
                list_state.select_prev_or_last();
            }
            KeyCode::Down => {
                list_state.select_next_or_first(Verdict::VARIANT_COUNT + 2);
            }
            KeyCode::Esc => {
                self.modal = None;
            }
            KeyCode::Enter => {
                let Some(selected) = list_state.selected() else {
                    return;
                };

                let filter = match selected {
                    0..Verdict::VARIANT_COUNT => VerdictFilter::Eq(
                        Verdict::variants()
                            .enumerate()
                            .find(|(i, _)| *i == selected)
                            .expect("invalid verdict")
                            .1,
                    ),
                    Verdict::VARIANT_COUNT => VerdictFilter::Unset,
                    _ => VerdictFilter::None,
                };

                self.list.set_verdict(filter);
                self.modal = None;
            }
            KeyCode::Char('1') => {
                self.list.set_verdict(VerdictFilter::Eq(Verdict::Yes));
                self.modal = None;
            }
            KeyCode::Char('2') => {
                self.list.set_verdict(VerdictFilter::Eq(Verdict::No));
                self.modal = None;
            }
            KeyCode::Char('3') => {
                self.list.set_verdict(VerdictFilter::Eq(Verdict::Ignore));
                self.modal = None;
            }
            KeyCode::Char('4') => {
                self.list.set_verdict(VerdictFilter::Unset);
                self.modal = None;
            }
            KeyCode::Char('5') => {
                self.list.set_verdict(VerdictFilter::None);
                self.modal = None;
            }
            _ => {}
        }
    }

    fn handle_filter_group(&mut self, code: KeyCode, mut tx: Tx<Self>) {
        let buffer = match self.modal.as_mut() {
            Some(RelatedModal::FilterGroup { buffer }) => buffer,
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
                tx.send(self.list.set_group_name(GroupName::from_str(buffer).ok()));
                self.modal = None;
            }
            _ => {}
        }
    }

    fn handle_main(&mut self, code: KeyCode, mut tx: Tx<Self>, quit: &mut bool) {
        macro_rules! load_article_info {
            () => {
                let Some(entry) = self.list.selected() else {
                    return;
                };
                tx.send(RelatedMessage::LoadArticleInfo {
                    article_name: entry.article_name.clone(),
                });
            };
        }
        match code {
            KeyCode::Up => {
                self.list.select_prev_or_last();
            }
            KeyCode::Down => {
                self.list.select_next_or_first();
            }
            KeyCode::Left => {
                self.list.select_prev_or_last();
                if self.article_info.is_some() {
                    load_article_info!();
                }
            }
            KeyCode::Right => {
                self.list.select_next_or_first();
                if self.article_info.is_some() {
                    load_article_info!();
                }
            }
            KeyCode::Home => {
                self.list.select_first();
            }
            KeyCode::End => {
                self.list.select_last();
            }
            KeyCode::PageUp => {
                let Some(skip) = self.list_block_inner_height else {
                    return;
                };
                let new_idx = self
                    .list
                    .selected_idx()
                    .map(|i| i.saturating_sub(skip))
                    .unwrap_or_default();
                self.list.state().select(Some(new_idx));
            }
            KeyCode::PageDown => {
                let Some(skip) = self.list_block_inner_height else {
                    return;
                };
                let max = self.list.len_including_virtual() - 1;
                let idx = self.list.selected_idx().unwrap_or_default();
                self.list.state().select(Some((idx + skip).min(max)));
            }
            KeyCode::Enter if self.list.selected_load_more() => {
                tx.send(self.list.load());
            }
            KeyCode::Enter => {
                load_article_info!();
            }
            KeyCode::Esc if self.article_info.is_some() => {
                self.article_info = None;
            }
            KeyCode::Esc => {
                *quit = true;
            }
            KeyCode::Char('g') => {
                self.modal = Some(RelatedModal::FilterGroup {
                    buffer: self
                        .list
                        .group_name()
                        .map(|g| g.to_string())
                        .unwrap_or_default(),
                })
            }
            KeyCode::Char('f') => {
                self.modal = Some(RelatedModal::FilterVerdict {
                    list_state: ListState::default(),
                });
            }
            KeyCode::Char('w') => {
                let Some(entry) = self.list.selected() else {
                    return;
                };
                self.modal = Some(RelatedModal::SetVerdict {
                    article_name: entry.article_name.clone(),
                    list_state: ListState::default(),
                })
            }
            KeyCode::Char('o') => {
                let Some(article_info) = self.article_info.as_ref() else {
                    return;
                };
                tx.send(RelatedMessage::OpenUrlInBrowser {
                    url: article_info.url().clone(),
                });
            }
            KeyCode::Char('q') => {
                *quit = true;
            }
            _ => {}
        }
    }
}

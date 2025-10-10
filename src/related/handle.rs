use super::{RelatedApp, RelatedMessage, render::RelatedModal};
use crate::{app::Messenger, database::Verdict, name::GroupName, render::list::ListStateExt};
use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use ratatui::widgets::ListState;
use std::str::FromStr;

impl RelatedApp {
    pub fn handle_impl(
        &mut self,
        event: Event,
        messenger: Messenger<Self>,
        quit: &mut bool,
    ) -> Result<()> {
        let Some(code) = event.as_key_press_event().map(|e| e.code) else {
            return Ok(());
        };
        match &self.modal {
            Some(RelatedModal::SetVerdict { .. }) => self.handle_set_verdict(code, messenger),
            Some(RelatedModal::SetGroup { .. }) => self.handle_set_group(code, messenger),
            None => self.handle_main(code, quit),
        }
        Ok(())
    }

    fn handle_set_verdict(&mut self, code: KeyCode, messenger: Messenger<Self>) {
        let (article_name, list_state) = match self.modal.as_mut() {
            Some(RelatedModal::SetVerdict {
                article_name,
                list_state,
            }) => (article_name, list_state),
            _ => unreachable!(),
        };
        let mut set_verdict = |verdict: Option<Verdict>| {
            messenger.send(RelatedMessage::SetVerdict {
                article_name: article_name.clone(),
                verdict,
            });
            if let Some(entry) = self
                .list_state
                .selected()
                .and_then(|i| self.list.get_mut(i))
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

    fn handle_set_group(&mut self, code: KeyCode, messenger: Messenger<Self>) {
        let buffer = match self.modal.as_mut() {
            Some(RelatedModal::SetGroup { buffer }) => buffer,
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
                messenger.send(self.list.set_group_name(GroupName::from_str(buffer).ok()));
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
                self.list_state.select_next_or_first(self.list.len());
            }
            KeyCode::Char('/') => {
                self.modal = Some(RelatedModal::SetGroup {
                    buffer: self
                        .list
                        .group_name()
                        .map(|g| g.to_string())
                        .unwrap_or_default(),
                })
            }
            KeyCode::Char('w') => {
                let Some(entry) = self.list_state.selected().and_then(|i| self.list.get(i)) else {
                    return;
                };
                self.modal = Some(RelatedModal::SetVerdict {
                    article_name: entry.article_name.clone(),
                    list_state: ListState::default(),
                })
            }
            KeyCode::Char('q') => {
                *quit = true;
            }
            _ => {}
        }
    }
}

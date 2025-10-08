mod render;

use crate::{app::App, name::ArticleName};
use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use ratatui::{prelude::*, widgets::ListState};

pub struct RelatedApp {
    orig_article_name: ArticleName,
    list_state: ListState,
}

impl RelatedApp {
    pub fn new(orig_article_name: ArticleName) -> Self {
        Self {
            orig_article_name,
            list_state: ListState::default(),
        }
    }
}

impl App for RelatedApp {
    async fn tick(&mut self) -> Result<()> {
        Ok(())
    }

    async fn handle(&mut self, event: Event, quit: &mut bool) -> Result<()> {
        match event.as_key_press_event().map(|event| event.code) {
            Some(KeyCode::Char('q')) => {
                *quit = true;
            }
            _ => {}
        }
        Ok(())
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        render::main(
            RelatedRenderer {
                orig_article_name: &self.orig_article_name,
                list_state: &mut self.list_state,
            },
            area,
            buf,
        );
    }
}

struct RelatedRenderer<'a> {
    orig_article_name: &'a ArticleName,
    list_state: &'a mut ListState,
}

mod crawl;
mod render;

use crate::{app::App, crawl::Crawler, database::Database, loader::Loader, name::ArticleName};
use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use ratatui::{prelude::*, widgets::ListState};

pub struct RelatedApp {
    related_articles: Vec<ArticleName>,
    related_loader: Loader<RelatedLoaderContext, Vec<ArticleName>>,
    related_loader_finished: bool,
    orig_article_name: ArticleName,
    list_state: ListState,
}

impl RelatedApp {
    pub fn new(crawler: Crawler, database: Database, orig_article_name: ArticleName) -> Self {
        Self {
            related_articles: Vec::new(),
            related_loader: Loader::new(
                RelatedLoaderContext::new(crawler, database, orig_article_name.clone()),
                |_ctx| async { vec![] },
            ),
            related_loader_finished: false,
            orig_article_name,
            list_state: ListState::default(),
        }
    }
}

impl App for RelatedApp {
    async fn tick(&mut self) -> Result<()> {
        if self.related_articles.is_empty()
            && !self.related_loader.is_loading()
            && !self.related_loader_finished
        {
            self.related_loader.start_loading();
        }

        if let Some(more_related_articles) = self.related_loader.read() {
            if more_related_articles.is_empty() {
                self.related_loader_finished = true;
            } else {
                self.related_articles.extend(more_related_articles);
            }
        }

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

struct RelatedLoaderContext {
    crawler: Crawler,
    database: Database,
    orig_article_name: ArticleName,
    page: u8,
}

impl RelatedLoaderContext {
    fn new(crawler: Crawler, database: Database, orig_article_name: ArticleName) -> Self {
        Self {
            crawler,
            database,
            orig_article_name,
            page: 1,
        }
    }
}

struct RelatedRenderer<'a> {
    orig_article_name: &'a ArticleName,
    list_state: &'a mut ListState,
}

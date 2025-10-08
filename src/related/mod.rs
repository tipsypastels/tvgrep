mod crawl;
mod render;

use self::crawl::RelatedCrawl;
use crate::{
    app::App,
    crawl::Crawler,
    database::Database,
    load::{Load, Loader},
    name::ArticleName,
};
use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use ratatui::{prelude::*, widgets::ListState};

pub struct RelatedApp {
    orig_article_name: ArticleName,
    list_state: ListState,
    list_entries: Vec<RelatedEntry>,
    list_loader: Loader<RelatedLoad>,
    list_loader_finished: bool,
}

impl RelatedApp {
    pub fn new(crawler: Crawler, database: Database, orig_article_name: ArticleName) -> Self {
        Self {
            orig_article_name: orig_article_name.clone(),
            list_state: ListState::default(),
            list_entries: Vec::new(),
            list_loader: Loader::new(RelatedLoad::new(crawler, database, orig_article_name)),
            list_loader_finished: false,
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

struct RelatedEntry {
    name: ArticleName,
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
        let vec = self.crawler.crawl(crawl).await?;
        let vec = vec.into_iter().map(|name| RelatedEntry { name }).collect();

        self.page += 1;
        Ok(vec)
    }
}

struct RelatedRenderer<'a> {
    orig_article_name: &'a ArticleName,
    list_state: &'a mut ListState,
}

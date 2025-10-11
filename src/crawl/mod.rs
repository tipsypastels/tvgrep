pub mod article;

use anyhow::{Context, Result};
use reqwest::Client;
use scraper::Html;
use std::borrow::Cow;

pub trait Crawl {
    type Output;

    fn url(&self) -> Cow<str>;
    fn crawl(&self, url: Cow<str>, html: Html) -> Result<Self::Output>;
}

#[derive(Clone)]
pub struct Crawler {
    client: Client,
}

impl Crawler {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn crawl<C: Crawl>(&self, crawl: C) -> Result<C::Output> {
        let url = crawl.url();
        let request = self.client.get(url.as_ref()).header("User-Agent", "tvgrep");
        let response = request.send().await.context("network error")?;
        let response = response.error_for_status()?;
        let text = response.text().await?;
        let html = Html::parse_document(&text);

        crawl.crawl(url, html)
    }
}

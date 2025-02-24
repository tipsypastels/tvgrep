mod article;
mod related;

use crate::{
    data::ArticleData,
    list::ArticleList,
    name::{ArticleName, GroupName},
};
use anyhow::{Context, Result};
use reqwest::Client;
use scraper::Html;

#[derive(Debug)]
pub struct Crawler {
    client: Client,
}

impl Crawler {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn article(&self, article: &ArticleName) -> Result<ArticleData> {
        article::crawl(&self.client, article)
            .await
            .context("article crawl error")
    }

    pub async fn related(
        &self,
        article: &ArticleName,
        group: Option<&GroupName>,
    ) -> Result<ArticleList> {
        related::crawl(&self.client, article, group)
            .await
            .context("related crawl error")
    }
}

async fn scrape(client: &Client, url: &str) -> Result<Html> {
    let request = client.get(url).header("User-Agent", "tvgrep");
    let response = request.send().await?;
    let response = response.error_for_status()?;
    let text = response.text().await?;
    let html = Html::parse_document(&text);
    Ok(html)
}

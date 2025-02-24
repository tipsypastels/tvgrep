use crate::{
    data::{ArticleData, ArticleSummaryData},
    name::ArticleName,
};
use anyhow::{Context, Result};
use kstring::KString;
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use std::sync::LazyLock;

static MAIN_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("#main-article").unwrap());
static TITLE_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("h1.entry-title").unwrap());

#[tracing::instrument(skip(client))]
pub async fn crawl(client: &Client, article: &ArticleName) -> Result<ArticleData> {
    let url = article.url();

    tracing::debug!(url, "crawling article");

    let html = super::scrape(client, &url).await?;
    let main = html.select(&MAIN_SEL).next().context("no main")?;

    let title = get_title(&html)?;
    let summary = get_summary(main);

    Ok(ArticleData {
        url: KString::from_string(url),
        title: KString::from_ref(title),
        summary,
    })
}

fn get_title(html: &Html) -> Result<&str> {
    html.select(&TITLE_SEL)
        .next()
        .context("no title")?
        .text()
        .filter_map(|s| {
            let s = s.trim();
            (!s.is_empty()).then_some(s)
        })
        .next()
        .context("empty title")
}

// TODO: Some summaries may not be in p tags.
fn get_summary(main: ElementRef) -> ArticleSummaryData {
    let mut out = ArticleSummaryData::builder();

    for child in main.children() {
        let Some(element_ref) = ElementRef::wrap(child) else {
            continue;
        };
        let element = element_ref.value();
        match element.name() {
            "hr" if element.attr("data-format").is_some() => break,
            "p" => {
                out.push_para();
                for s in element_ref.text() {
                    out.push_str(s);
                }
            }
            _ => {}
        }
    }

    out.build()
}

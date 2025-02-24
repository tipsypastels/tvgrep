use crate::{data::ArticleData, name::ArticleName};
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
        summary: KString::from_string(summary),
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
fn get_summary(main: ElementRef) -> String {
    const LIMIT_LEN: usize = 2000;

    let mut out = String::new();

    for child in main.children() {
        let Some(element_ref) = ElementRef::wrap(child) else {
            continue;
        };
        let element = element_ref.value();
        match element.name() {
            "hr" if element.attr("data-format").is_some() => break,
            "p" => {
                let mut has_text = false;

                for s in element_ref.text() {
                    has_text = true;
                    out.push_str(s);
                }

                if has_text {
                    out.push_str("\n\n");
                }

                if out.len() > LIMIT_LEN {
                    break;
                }
            }
            _ => {}
        }
    }

    while !out.is_empty() && out.as_bytes()[out.len() - 1] == b'\n' {
        out.pop();
    }

    out
}

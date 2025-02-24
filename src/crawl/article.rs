use crate::{data::ArticleData, name::ArticleName};
use anyhow::Result;
use kstring::KString;
use reqwest::Client;
use scraper::{ElementRef, Selector};

const TITLE_SELECTOR: &str = "h1.entry-title";
const MAIN_SELECTOR: &str = "#main-article";

#[tracing::instrument(skip(client))]
pub async fn crawl(client: &Client, article: &ArticleName) -> Result<ArticleData> {
    let url = article.url();

    tracing::debug!(url, "crawling article");

    let html = super::scrape(client, &url).await?;

    let title_selector = Selector::parse(TITLE_SELECTOR).unwrap();
    let title_node = html.select(&title_selector).next().unwrap();
    let title = title_node.text().next().unwrap().trim();

    let main_selector = Selector::parse(MAIN_SELECTOR).unwrap();
    let main = html.select(&main_selector).next().unwrap();
    let summary = get_summary(main);

    Ok(ArticleData {
        name: article.clone(),
        title: KString::from_ref(title),
        summary: KString::from_string(summary),
    })
}

// TODO: Some summaries may not be in p tags.
fn get_summary(main: ElementRef) -> String {
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
            }
            _ => {}
        }
    }

    while !out.is_empty() && out.as_bytes()[out.len() - 1] == b'\n' {
        out.pop();
    }

    out
}

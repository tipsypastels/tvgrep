use super::Crawl;
use crate::{name::ArticleName, url::ArticleUrl};
use anyhow::{Context, Result};
use kstring::KString;
use scraper::{ElementRef, Html, Selector};
use std::{borrow::Cow, sync::LazyLock};

static MAIN_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("#main-article").unwrap());
static TITLE_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("h1.entry-title").unwrap());
static TROPE_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("li > a.twikilink:first-child").unwrap());

pub struct ArticleInfo<Body> {
    pub url: KString,
    pub name: ArticleName,
    pub title: KString,
    pub summary: KString,
    pub body: Body,
}

pub struct ArticleCrawl<CrawlBody> {
    pub article_name: ArticleName,
    pub crawl_body: CrawlBody,
}

impl<CrawlBody> Crawl for ArticleCrawl<CrawlBody>
where
    CrawlBody: ArticleCrawlBody,
{
    type Output = ArticleInfo<CrawlBody::Body>;

    fn url(&self) -> Cow<str> {
        self.article_name.url().into()
    }

    fn crawl(&self, url: Cow<str>, html: Html) -> Result<Self::Output> {
        let main = html.select(&MAIN_SEL).next().context("no main")?;
        let title = get_title(&html)?;
        let summary = get_summary(main);
        let body = self.crawl_body.crawl_body(&html)?;

        Ok(ArticleInfo {
            // TODO: Consider using `KStringCow`.
            url: KString::from_ref(&url),
            name: self.article_name.clone(),
            title: KString::from_ref(title),
            summary: KString::from_string(summary),
            body,
        })
    }
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
    let mut out = String::new();

    for child in main.children() {
        let Some(element_ref) = ElementRef::wrap(child) else {
            continue;
        };
        let element = element_ref.value();
        match element.name() {
            "hr" if element.attr("data-format").is_some() => break,
            "p" => {
                if !out.is_empty() {
                    out.push_str("\n\n");
                }
                for s in element_ref.text() {
                    out.push_str(s);
                }
            }
            _ => {}
        }
    }

    out
}

pub trait ArticleCrawlBody {
    type Body;
    fn crawl_body(&self, html: &Html) -> Result<Self::Body>;
}

pub struct ArticleCrawlSingleTrope(pub ArticleName);

// TODO: Generalize to find links anywhere, make this an enum.
pub struct ArticleSingleTropeBody {
    pub article_name: ArticleName,
    pub text: Option<KString>,
}

impl ArticleCrawlBody for ArticleCrawlSingleTrope {
    type Body = ArticleSingleTropeBody;

    fn crawl_body(&self, html: &Html) -> Result<Self::Body> {
        let Some(trope_node) = html.select(&TROPE_SEL).find(|node| {
            node.attr("href")
                .is_some_and(|url| self.0.matches_relative_url(url))
        }) else {
            return Ok(ArticleSingleTropeBody {
                article_name: self.0.clone(),
                text: None,
            });
        };

        let li_node = trope_node.parent().context("trope_node lacks parent")?;
        let li_node = ElementRef::wrap(li_node).context("trope_node lacks parent")?;
        let li_node_text = li_node
            .text()
            .filter(|s| !s.trim().is_empty())
            .enumerate()
            .map(|(i, s)| {
                if i == 1 {
                    s.strip_prefix(": ").unwrap_or(s)
                } else {
                    s
                }
            })
            .skip(1) // skip trope name
            .collect::<String>();

        let text = Some(KString::from_string(li_node_text));

        Ok(ArticleSingleTropeBody {
            article_name: self.0.clone(),
            text,
        })
    }
}

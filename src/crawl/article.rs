use super::Crawl;
use crate::{name::ArticleName, url::ArticleUrl};
use anyhow::{Context, Result};
use kstring::KString;
use scraper::{ElementRef, Html, Selector};
use std::{borrow::Cow, ops::Range, sync::LazyLock};

static MAIN_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("#main-article").unwrap());
static TITLE_SEL_TYPE_1: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("h1.entry-title > .wrapped_title").unwrap());
static TITLE_SEL_TYPE_2: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".wrapped_title > h1.entry-title strong").unwrap());
static FIRST_A_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("a.twikilink:first-of-type").unwrap());

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
    let (element, is_type1) = html
        .select(&TITLE_SEL_TYPE_1)
        .next()
        .map(|e| (e, true))
        .or_else(|| html.select(&TITLE_SEL_TYPE_2).next().map(|e| (e, false)))
        .context("no title")?;
    element
        .text()
        .skip(if is_type1 { 0 } else { 1 })
        .filter_map(|s| {
            let s = s.trim();
            (!s.is_empty()).then_some(s)
        })
        .last()
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

pub enum ArticleSingleTropeBody {
    TopLevel {
        article_name: ArticleName,
        text: KString,
    },
    InOther {
        other_article_name: ArticleName,
        text: KString,
        own_article_url_range: Option<Range<usize>>,
    },
    Elsewhere {
        nearest_block_parent_text: KString,
        url_range: Option<Range<usize>>,
    },
}

// TODO: Figure out why spacing is sometimes missing around non-highlighted trope links?
impl ArticleCrawlBody for ArticleCrawlSingleTrope {
    type Body = ArticleSingleTropeBody;

    fn crawl_body(&self, html: &Html) -> Result<Self::Body> {
        let a_sel_string = format!("a.twikilink[href=\"{}\"]", self.0.relative_url());
        let a_sel = Selector::parse(&a_sel_string).unwrap();
        let a = html.select(&a_sel).next().context("missing trope")?;

        let a_text_range = |in_text: &KString| {
            let a_text = a.text().collect::<String>();
            in_text.find(&a_text).map(|n| n..(n + a_text.len()))
        };

        if let Some((li, first_a)) = a
            .ancestors()
            .find_map(|node| {
                let element = ElementRef::wrap(node)?;
                (element.value().name() == "li").then_some(element)
            })
            .and_then(|li| {
                let first_a = li.select(&FIRST_A_SEL).next()?;
                Some((li, first_a))
            })
        {
            let text = li
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
                .collect::<String>()
                .into();

            if a == first_a {
                return Ok(ArticleSingleTropeBody::TopLevel {
                    article_name: self.0.clone(),
                    text,
                });
            }

            let first_a_href = first_a.attr("href").context("first a missing href")?;
            let other_article_name = ArticleName::from_relative_url(first_a_href)?;
            let own_article_url_range = a_text_range(&text);

            return Ok(ArticleSingleTropeBody::InOther {
                other_article_name,
                text,
                own_article_url_range,
            });
        };

        let nearest_block_parent = a
            .ancestors()
            .find_map(|node| {
                let element = ElementRef::wrap(node)?;
                // Add more block elements if they're needed.
                matches!(element.value().name(), "li" | "p" | "div").then_some(element)
            })
            .context("could not find block parent of a")?;

        let nearest_block_parent_text = nearest_block_parent.text().collect::<String>().into();
        let url_range = a_text_range(&nearest_block_parent_text);

        Ok(ArticleSingleTropeBody::Elsewhere {
            nearest_block_parent_text,
            url_range,
        })
    }
}

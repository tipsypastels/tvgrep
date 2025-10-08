use crate::{crawl::Crawl, name::ArticleName, url::ArticleUrl};
use anyhow::{Context, Result};
use scraper::{Html, Selector};
use std::{borrow::Cow, sync::LazyLock};

static LISTING_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("ul.no-bullets > li > a[href]").unwrap());

pub struct RelatedCrawl {
    article_name: ArticleName,
    page: u8,
}

impl RelatedCrawl {
    pub fn new(article_name: ArticleName, page: u8) -> Self {
        Self { article_name, page }
    }
}

impl Crawl for RelatedCrawl {
    // TODO: Return an iterator wrapper?
    type Output = Vec<ArticleName>;

    fn url(&self) -> Cow<str> {
        // TODO: Allow filtering group.
        self.article_name
            .related_url()
            .page(self.page)
            .to_string()
            .into()
    }

    fn crawl(&self, html: Html) -> Result<Self::Output> {
        html.select(&LISTING_SEL)
            .map(|link| {
                let url = link.attr("href").context("link has no href")?;
                ArticleName::from_relative_url(url).context("invalid relative url")
            })
            .collect()
    }
}

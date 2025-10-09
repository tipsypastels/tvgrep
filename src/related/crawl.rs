use crate::{
    crawl::Crawl,
    name::{ArticleName, GroupName},
    url::ArticleUrl,
};
use anyhow::{Context, Result};
use scraper::{Html, Selector};
use std::{borrow::Cow, sync::LazyLock};

static LISTING_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("ul.no-bullets > li > a[href]").unwrap());

pub struct RelatedCrawl {
    pub article_name: ArticleName,
    pub group_name: Option<GroupName>,
    pub page: u8,
}

impl Crawl for RelatedCrawl {
    type Output = Vec<ArticleName>;

    fn url(&self) -> Cow<str> {
        self.article_name
            .related_url()
            .group(self.group_name.as_ref())
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

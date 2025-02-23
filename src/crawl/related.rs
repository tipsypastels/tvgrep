use crate::{
    name::{ArticleName, GroupName},
    url::{article_related_url, get_article_from_url},
};
use anyhow::{Context, Result};
use reqwest::Client;
use scraper::Selector;

const LISTING_SELECTOR: &str = ".examples-header + ul > li > a";

pub async fn crawl(
    client: &Client,
    article: &ArticleName,
    group: Option<&GroupName>,
) -> Result<Vec<ArticleName>> {
    RelatedCrawler {
        client,
        article,
        group,
    }
    .crawl()
    .await
}

struct RelatedCrawler<'a> {
    client: &'a Client,
    article: &'a ArticleName,
    group: Option<&'a GroupName>,
}

impl RelatedCrawler<'_> {
    async fn crawl(self) -> Result<Vec<ArticleName>> {
        let mut out = Vec::new();
        let mut page = 1u8;

        tracing::debug!("starting page loop");
        loop {
            tracing::debug!(page, "starting page");

            let cur_len = out.len();
            self.crawl_page(&mut out, page).await?;

            if cur_len == out.len() {
                tracing::debug!(page, "done");
                break;
            }
            page += 1;
        }

        Ok(out)
    }

    async fn crawl_page(&self, out: &mut Vec<ArticleName>, page: u8) -> Result<()> {
        tracing::debug!(page, "fetching");

        let url = article_related_url(self.article)
            .group(self.group)
            .page(page)
            .build();

        let html = super::scrape(self.client, &url).await?;

        // If we were signed in this would not work because there's an additional <a> to edit inside each <li>.
        let selector = Selector::parse(LISTING_SELECTOR).unwrap();
        let links = html.select(&selector);

        for link in links {
            let url = link.attr("href").context("link has no url")?;
            let article = get_article_from_url(url).context("invalid url")?;

            tracing::debug!(%article, "article");
            out.push(article);
        }

        Ok(())
    }
}

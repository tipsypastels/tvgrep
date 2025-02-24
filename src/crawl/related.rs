use crate::{
    list::ArticleList,
    name::{ArticleName, GroupName},
};
use anyhow::{Context, Result};
use reqwest::Client;
use scraper::Selector;
use tracing::{Span, debug, field, instrument, trace};

// If we were signed in this would not work because there's an additional <a> to edit inside each <li>.
// TODO: Doesn't work without a group, no header.
const LISTING_SELECTOR: &str = ".examples-header + ul > li > a";

pub async fn crawl(
    client: &Client,
    article: &ArticleName,
    group: Option<&GroupName>,
) -> Result<ArticleList> {
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
    #[instrument(skip_all, fields(article = %self.article, group = field::Empty))]
    async fn crawl(self) -> Result<ArticleList> {
        if let Some(group) = &self.group {
            Span::current().record("group", field::display(group));
        }

        let mut out = ArticleList::new();
        let mut page = 1u8;

        loop {
            debug!(page, "loading page");

            let cur_len = out.len();
            self.crawl_page(&mut out, page).await?;

            let addl_len = out.len() - cur_len;
            debug!(page, len = addl_len, "loaded");

            if addl_len == 0 {
                break;
            }
            page += 1;
        }

        Ok(out)
    }

    #[instrument(skip(self, out))]
    async fn crawl_page(&self, out: &mut ArticleList, page: u8) -> Result<()> {
        let url = self
            .article
            .related_url()
            .group(self.group)
            .page(page)
            .build();

        debug!(url, "crawling listing");

        let html = super::scrape(self.client, &url).await?;

        let selector = Selector::parse(LISTING_SELECTOR).unwrap();
        let links = html.select(&selector);

        for link in links {
            let url = link.attr("href").context("link has no url")?;
            let article = ArticleName::from_url(url).context("invalid url")?;

            trace!(%article);
            out.push_assume_sorted(article);
        }

        Ok(())
    }
}

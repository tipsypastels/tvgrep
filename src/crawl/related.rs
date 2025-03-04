use crate::{
    list::ArticleList,
    name::{ArticleName, GroupName},
};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use scraper::{Html, Selector};
use std::sync::{LazyLock, OnceLock};

static COUNT_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("p > strong").unwrap());

// If we were signed in this would not work because there's an additional <a> to edit inside each <li>.
// TODO: Doesn't work without a group, no header.
static LISTING_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".examples-header + ul > li > a").unwrap());

pub async fn crawl(
    client: &Client,
    article: &ArticleName,
    group: Option<&GroupName>,
) -> Result<ArticleList> {
    RelatedCrawler {
        client,
        article,
        group,
        progress: OnceLock::new(),
    }
    .crawl()
    .await
}

struct RelatedCrawler<'a> {
    client: &'a Client,
    article: &'a ArticleName,
    group: Option<&'a GroupName>,
    progress: OnceLock<CrawlerProgress>,
}

impl RelatedCrawler<'_> {
    async fn crawl(self) -> Result<ArticleList> {
        let mut out = ArticleList::new();
        let mut page = 1u8;

        loop {
            let cur_len = out.len();
            self.crawl_page(&mut out, page).await?;

            let addl_len = out.len() - cur_len;

            if addl_len == 0 {
                break;
            }
            page += 1;
        }

        if let Some(progress) = self.progress.get() {
            progress.finish();
        }

        Ok(out)
    }

    async fn crawl_page(&self, out: &mut ArticleList, page: u8) -> Result<()> {
        let url = self
            .article
            .related_url()
            .group(self.group)
            .page(page)
            .build();

        let html = super::scrape(self.client, &url).await?;
        let progress = self.get_or_init_progress(&html);
        let links = html.select(&LISTING_SEL);

        progress.set_page(page);

        for link in links {
            let url = link.attr("href").context("link has no url")?;
            let article = ArticleName::from_url(url).context("invalid url")?;

            out.push_assume_sorted(article);
            progress.inc();
        }

        Ok(())
    }

    fn get_or_init_progress(&self, html: &Html) -> &CrawlerProgress {
        self.progress.get_or_init(|| CrawlerProgress::new(html))
    }
}

#[derive(Debug)]
struct CrawlerProgress(Option<ProgressBar>);

impl CrawlerProgress {
    fn new(html: &Html) -> Self {
        Self(find_total_count(html).map(create_progress))
    }

    fn set_page(&self, page: u8) {
        if let Some(pb) = &self.0 {
            pb.set_message(format!("Page {page}"));
        }
    }

    fn inc(&self) {
        if let Some(pb) = &self.0 {
            pb.inc(1);
        }
    }

    fn finish(&self) {
        if let Some(pb) = &self.0 {
            pb.finish_and_clear();
        }
    }
}

fn create_progress(len: u64) -> ProgressBar {
    ProgressBar::new(len)
        .with_style(ProgressStyle::with_template("{msg} {wide_bar} {pos}/{len}").unwrap())
}

fn find_total_count(html: &Html) -> Option<u64> {
    html.select(&COUNT_SEL).next()?.text().next()?.parse().ok()
}

use super::*;
use anyhow::{Result, ensure};
use std::fmt;

const PMWIKI_BASE: &str = "https://tvtropes.org/pmwiki";
const REL_ARTICLE_URL_BASE: &str = "/pmwiki/pmwiki.php/";

pub fn article_url(article: &ArticleName) -> String {
    format!("{PMWIKI_BASE}/pmwiki.php/{article}")
}

pub fn related_url(article: &ArticleName) -> RelatedUrlBuilder {
    RelatedUrlBuilder {
        article,
        group: None,
        page: None,
    }
}

pub fn article_from_url(url: &str) -> Result<ArticleName> {
    ensure!(url.starts_with(REL_ARTICLE_URL_BASE));
    url[REL_ARTICLE_URL_BASE.len()..].parse()
}

#[derive(Debug)]
pub struct RelatedUrlBuilder<'a> {
    article: &'a ArticleName,
    group: Option<&'a GroupName>,
    page: Option<u8>,
}

impl<'a> RelatedUrlBuilder<'a> {
    pub fn group(&mut self, group: Option<&'a GroupName>) -> &mut Self {
        self.group = group;
        self
    }

    pub fn page(&mut self, page: u8) -> &mut Self {
        self.page = Some(page);
        self
    }

    pub fn build(&self) -> String {
        format!("{self}")
    }
}

impl fmt::Display for RelatedUrlBuilder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{PMWIKI_BASE}/relatedsearch.php?term={}", self.article)?;
        if let Some(group) = self.group {
            write!(f, "&groupname_search={group}")?;
        }
        if let Some(page) = self.page {
            write!(f, "&page={page}")?;
        }
        Ok(())
    }
}

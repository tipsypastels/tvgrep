use crate::name::{ArticleName, GroupName};
use anyhow::{Result, ensure};
use std::fmt;

const PMWIKI_BASE: &str = "https://tvtropes.org/pmwiki";
const RELATIVE_ARTICLE_URL_BASE: &str = "/pmwiki/pmwiki.php/";

pub trait ArticleUrl: Sized {
    fn url(&self) -> String;
    fn relative_url(&self) -> String;
    fn related_url(&self) -> ArticleRelatedUrlBuilder;

    fn from_relative_url(url: &str) -> Result<Self>;
    fn matches_relative_url(&self, url: &str) -> bool;
}

impl ArticleUrl for ArticleName {
    fn url(&self) -> String {
        format!("{PMWIKI_BASE}/pmwiki.php/{self}")
    }

    fn relative_url(&self) -> String {
        format!("/pmwiki/pmwiki.php/{self}")
    }

    fn related_url(&self) -> ArticleRelatedUrlBuilder {
        ArticleRelatedUrlBuilder {
            article: self,
            group: None,
            page: None,
        }
    }

    fn from_relative_url(url: &str) -> Result<Self> {
        ensure!(url.starts_with(RELATIVE_ARTICLE_URL_BASE));
        url[RELATIVE_ARTICLE_URL_BASE.len()..].parse()
    }

    fn matches_relative_url(&self, url: &str) -> bool {
        url.starts_with(RELATIVE_ARTICLE_URL_BASE)
            && self == &url[RELATIVE_ARTICLE_URL_BASE.len()..]
    }
}

pub struct ArticleRelatedUrlBuilder<'a> {
    article: &'a ArticleName,
    group: Option<&'a GroupName>,
    page: Option<u8>,
}

impl<'a> ArticleRelatedUrlBuilder<'a> {
    pub fn group(&mut self, group: Option<&'a GroupName>) -> &mut Self {
        self.group = group;
        self
    }

    pub fn page(&mut self, page: u8) -> &mut Self {
        self.page = Some(page);
        self
    }
}

impl fmt::Display for ArticleRelatedUrlBuilder<'_> {
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

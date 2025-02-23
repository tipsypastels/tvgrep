use crate::name::{ArticleName, GroupName};
use anyhow::{Result, ensure};
use std::fmt;

const PMWIKI: &str = "https://tvtropes.org/pmwiki";
const RELATIVE_ARTICLE_URL_BASE: &str = "/pmwiki/pmwiki.php/";

pub fn article_url(article: &ArticleName) -> String {
    format!("{PMWIKI}/pmwiki.php/{article}")
}

pub fn article_related_url(article: &ArticleName) -> ArticleRelatedUrlBuilder {
    ArticleRelatedUrlBuilder {
        article,
        group: None,
        page: None,
    }
}

pub fn get_article_from_url(url: &str) -> Result<ArticleName> {
    ensure!(url.starts_with(RELATIVE_ARTICLE_URL_BASE));
    let name = &url[RELATIVE_ARTICLE_URL_BASE.len()..];
    Ok(name.parse()?)
}

#[derive(Debug)]
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

    pub fn build(&self) -> String {
        format!("{self}")
    }
}

impl fmt::Display for ArticleRelatedUrlBuilder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{PMWIKI}/relatedsearch.php?term={}", self.article)?;
        if let Some(group) = self.group {
            write!(f, "&groupname_search={group}")?;
        }
        if let Some(page) = self.page {
            write!(f, "&page={page}")?;
        }
        Ok(())
    }
}

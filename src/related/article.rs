use crate::{
    crawl::article::{ArticleInfo, ArticleSingleTropeBody},
    name::ArticleName,
};
use kstring::KString;

pub struct RelatedArticleInfo {
    article_info: ArticleInfo<ArticleSingleTropeBody>,
}

impl RelatedArticleInfo {
    pub fn new(article_info: ArticleInfo<ArticleSingleTropeBody>) -> Self {
        Self { article_info }
    }

    pub fn url(&self) -> &KString {
        &self.article_info.url
    }

    pub fn title(&self) -> &str {
        &self.article_info.title
    }

    pub fn summary(&self) -> &str {
        &self.article_info.summary
    }

    pub fn article_name(&self) -> &ArticleName {
        &self.article_info.name
    }
}

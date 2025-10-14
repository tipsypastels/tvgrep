use crate::{
    crawl::article::{ArticleInfo, ArticleSingleTropeBody},
    name::ArticleName,
};
use kstring::KString;

pub struct RelatedArticleInfo {
    article_info: ArticleInfo<ArticleSingleTropeBody>,
    tab: RelatedArticleInfoTab,
}

#[derive(Copy, Clone)]
pub enum RelatedArticleInfoTab {
    Summary,
    Trope,
}

impl RelatedArticleInfo {
    pub fn new(article_info: ArticleInfo<ArticleSingleTropeBody>) -> Self {
        Self {
            article_info,
            tab: RelatedArticleInfoTab::Summary,
        }
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

    pub fn trope(&self) -> &ArticleSingleTropeBody {
        &self.article_info.body
    }

    pub fn tab(&self) -> RelatedArticleInfoTab {
        self.tab
    }

    pub fn toggle_tab(&mut self) {
        self.tab = match self.tab {
            RelatedArticleInfoTab::Summary => RelatedArticleInfoTab::Trope,
            RelatedArticleInfoTab::Trope => RelatedArticleInfoTab::Summary,
        }
    }
}

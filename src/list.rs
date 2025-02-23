use crate::name::ArticleName;
use std::ops::Deref;

#[derive(Debug, Default)]
pub struct ArticleList(Vec<ArticleName>);

impl ArticleList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[allow(unused)]
    pub fn push(&mut self, article: ArticleName) {
        let index = match self.0.binary_search(&article) {
            Ok(i) | Err(i) => i,
        };
        self.0.insert(index, article);
    }

    pub fn push_assume_sorted(&mut self, article: ArticleName) {
        self.0.push(article);
    }
}

impl Deref for ArticleList {
    type Target = [ArticleName];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

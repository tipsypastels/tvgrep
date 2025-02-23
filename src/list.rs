use crate::name::ArticleName;
use ahash::RandomState;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{fmt, ops::Deref};

#[derive(Debug, Default, Deserialize, Serialize)]
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

impl fmt::Display for ArticleList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cur_group = None;

        for article in self.iter() {
            let group = &article.group;

            if cur_group.is_none() || cur_group.is_some_and(|cg| cg != group) {
                cur_group = Some(group);
                writeln!(f)?;
                writeln!(f, "{group}")?;
                writeln!(f)?;
            }

            writeln!(f, "\t{}", article.display_link())?;
        }

        writeln!(f)?;
        writeln!(f, "({} results)", self.len())?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleMap<T>(IndexMap<ArticleName, T, RandomState>);

impl<T> ArticleMap<T> {
    pub fn new() -> Self {
        Self(IndexMap::with_hasher(RandomState::new()))
    }

    pub fn has(&self, article: &ArticleName) -> bool {
        self.0.get(article).is_some()
    }

    pub fn get(&self, article: &ArticleName) -> Option<&T> {
        self.0.get(article)
    }

    pub fn insert(&mut self, article: ArticleName, value: T) {
        self.0.insert(article, value);
    }
}

impl<T> Default for ArticleMap<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

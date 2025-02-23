use crate::name::ArticleName;
use std::{fmt, ops::Deref};

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

            writeln!(f, "\t{}", article.display_with_url())?;
        }

        writeln!(f)?;
        writeln!(f, "({} results)", self.len())?;

        Ok(())
    }
}

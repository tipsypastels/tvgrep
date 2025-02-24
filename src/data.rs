use crate::term;
use console::style;
use kstring::KString;
use std::fmt;

#[derive(Debug)]
pub struct ArticleData {
    pub url: KString,
    pub title: KString,
    pub summary: KString,
}

impl fmt::Display for ArticleData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "=== {} ===", term::link(&self.title, &self.url))?;
        writeln!(f)?;
        writeln!(f, "{}", style(&self.summary).dim())
    }
}

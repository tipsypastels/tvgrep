use crate::term;
use console::style;
use kstring::KString;
use std::fmt;

#[derive(Debug)]
pub struct ArticleData<T = ()> {
    pub url: KString,
    pub title: KString,
    pub summary: ArticleSummaryData,
    pub tropes: T,
}

impl<T: fmt::Debug> fmt::Display for ArticleData<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "=== {} ===", term::link(&self.title, &self.url))?;
        writeln!(f)?;
        writeln!(f, "{:?}", self.tropes)?;
        writeln!(f)?;
        writeln!(f, "{}", style(&self.summary).dim())
    }
}

#[derive(Debug)]
pub struct ArticleSummaryData(KString);

impl ArticleSummaryData {
    pub fn builder() -> ArticleSummaryDataBuilder {
        ArticleSummaryDataBuilder(String::new())
    }
}

impl fmt::Display for ArticleSummaryData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const CUTOFF: usize = 2000;

        let mut len_written = 0usize;
        let mut addl_cutoff_paras = 0usize;

        for para in self.0.split("\n\n") {
            if para.is_empty() {
                continue;
            }

            if len_written > CUTOFF {
                addl_cutoff_paras += 1;
                continue;
            }

            if len_written > 0 {
                writeln!(f)?;
            }

            len_written += para.len();
            writeln!(f, "{para}")?;
        }

        if addl_cutoff_paras > 0 {
            writeln!(f)?;
            write!(f, "({addl_cutoff_paras} more paragraphs)")?;
        }

        Ok(())
    }
}

pub struct ArticleSummaryDataBuilder(String);

impl ArticleSummaryDataBuilder {
    pub fn push_para(&mut self) {
        if !self.0.is_empty() {
            self.0.push_str("\n\n");
        }
    }

    pub fn push_str(&mut self, para: &str) {
        self.0.push_str(para);
    }

    pub fn build(self) -> ArticleSummaryData {
        ArticleSummaryData(KString::from_string(self.0))
    }
}

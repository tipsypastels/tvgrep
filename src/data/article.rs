use crate::{Progress, term};
use console::style;
use kstring::KString;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct ArticleData<T = super::TropeDataStub> {
    pub url: KString,
    pub title: KString,
    pub summary: ArticleSummaryData,
    pub tropes: T,
}

impl<T: Display> ArticleData<T> {
    pub fn display_with_progress(&self, progress: Progress) -> impl Display {
        ArticleDataDisplay(Some(progress), self)
    }
}

impl<T: Display> Display for ArticleData<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ArticleDataDisplay(None, self).fmt(f)
    }
}

struct ArticleDataDisplay<'a, T>(Option<Progress>, &'a ArticleData<T>);

impl<T: Display> Display for ArticleDataDisplay<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;

        let link = term::link(&self.1.title, &self.1.url);

        if let Some(progress) = self.0 {
            writeln!(f, "=== {link} === {progress}")?;
        } else {
            writeln!(f, "=== {link} ===")?;
        }

        writeln!(f)?;
        writeln!(f, "{}", style(&self.1.summary).dim())?;
        writeln!(f, "{}", self.1.tropes)
    }
}

#[derive(Debug)]
pub struct ArticleSummaryData(KString);

impl ArticleSummaryData {
    pub fn builder() -> ArticleSummaryDataBuilder {
        ArticleSummaryDataBuilder(String::new())
    }
}

impl Display for ArticleSummaryData {
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
            writeln!(f, "({addl_cutoff_paras} more paragraphs)")?;
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

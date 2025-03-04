use crate::name::ArticleName;
use kstring::KString;
use std::fmt;

#[derive(Debug)]
pub struct TropeDataStub;

impl fmt::Display for TropeDataStub {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

#[derive(Debug)]
pub struct TropeDataSingle {
    pub trope: ArticleName,
    pub text: TropeDataSingleText,
}

#[derive(Debug)]
pub enum TropeDataSingleText {
    /// Trope has this text.
    Text(KString),
    /// Trope has no text.
    Blank,
    /// Trope was not found on the tropes list.
    /// This probably means it's a stray link somewhere else on the page.
    Missing,
}

impl fmt::Display for TropeDataSingle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let trope = self.trope.display_link();
        match &self.text {
            TropeDataSingleText::Text(text) => {
                writeln!(f, "{trope}: {text}")
            }
            TropeDataSingleText::Blank => {
                writeln!(f, "{trope}: {}", console::style("(no text)").dim())
            }
            TropeDataSingleText::Missing => {
                writeln!(f, "{trope}: {}", console::style("(trope missing)").on_red())
            }
        }
    }
}

#[derive(Debug)]
pub struct TropeDataFlatList(pub Vec<ArticleName>);

// TODO: Use the printer feature for this.
impl fmt::Display for TropeDataFlatList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for trope in &self.0 {
            writeln!(f, "{}", trope.display_link())?;
        }
        writeln!(f)
    }
}

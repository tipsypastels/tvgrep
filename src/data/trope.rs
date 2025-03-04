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
    pub desc: KString,
}

impl fmt::Display for TropeDataSingle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}: {}", self.trope.display_link(), self.desc)
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

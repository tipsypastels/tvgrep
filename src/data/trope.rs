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
pub struct TropeDataSingle(Option<TropeDataSingleInner>);

#[derive(Debug)]
pub struct TropeDataSingleInner {
    trope: ArticleName,
    desc: KString,
}

impl TropeDataSingle {
    pub fn new(trope: ArticleName, desc: KString) -> Self {
        Self(Some(TropeDataSingleInner { trope, desc }))
    }

    pub fn none() -> Self {
        Self(None)
    }
}

impl fmt::Display for TropeDataSingle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(inner) = &self.0 {
            writeln!(f, "{}: {}", inner.trope.display_link(), inner.desc)
        } else {
            writeln!(f, "{}", console::style("(no trope data)").on_red())
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

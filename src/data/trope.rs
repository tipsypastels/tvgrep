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
    pub text: Option<KString>,
}

impl fmt::Display for TropeDataSingle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let trope = self.trope.display_link();
        match &self.text {
            None => {
                writeln!(f, "{trope}: {}", console::style("(trope missing)").on_red())
            }
            Some(text) if text.is_empty() => {
                writeln!(f, "{trope}: {}", console::style("(trope blank)").dim())
            }
            Some(text) => {
                writeln!(f, "{trope}: {text}")
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

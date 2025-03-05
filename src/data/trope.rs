use crate::name::ArticleName;
use console::style;
use kstring::KString;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct TropeDataStub;

impl Display for TropeDataStub {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

#[derive(Debug)]
pub struct TropeDataSingle {
    pub trope: ArticleName,
    pub text: Option<KString>,
}

impl TropeDataSingle {
    pub fn display_text(&self) -> impl Display {
        struct DisplayText<'a>(Option<&'a str>);
        impl Display for DisplayText<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.0 {
                    None => write!(f, "{}", style("(trope missing)").on_red()),
                    Some("") => write!(f, "{}", style("(trope blank)").dim()),
                    Some(text) => write!(f, "{text}"),
                }
            }
        }
        DisplayText(self.text.as_deref())
    }
}

impl Display for TropeDataSingle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}: {}", self.trope.display_link(), self.display_text())
    }
}

#[derive(Debug)]
pub struct TropeDataFlatList(pub Vec<ArticleName>);

impl Display for TropeDataFlatList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for trope in &self.0 {
            writeln!(f, "{}", trope.display_link())?;
        }
        writeln!(f)
    }
}

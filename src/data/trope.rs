use crate::name::ArticleName;
use std::fmt;

#[derive(Debug)]
pub struct TropeDataStub;

impl fmt::Display for TropeDataStub {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

#[derive(Debug)]
pub struct TropeDataFlatList(pub Vec<ArticleName>);

// TODO: Use the printer feature for this.
impl fmt::Display for TropeDataFlatList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for article in &self.0 {
            writeln!(f, "{}", article.display_link())?;
        }
        writeln!(f)
    }
}

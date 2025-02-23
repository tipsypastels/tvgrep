mod group;
mod page;
mod url;

use anyhow::{Error, Result};
use kstring::KString;
use serde::{Deserialize, Deserializer};
use std::{fmt, str::FromStr};

pub use group::GroupName;
pub use page::PageName;
pub use url::RelatedUrlBuilder;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArticleName {
    pub group: GroupName,
    pub page: PageName,
}

impl ArticleName {
    pub fn from_url(url: &str) -> Result<Self> {
        url::article_from_url(url)
    }

    pub fn url(&self) -> String {
        url::article_url(self)
    }

    pub fn related_url(&self) -> RelatedUrlBuilder {
        url::related_url(self)
    }

    #[allow(unused)]
    pub fn display_without_main(&self) -> impl fmt::Display {
        DisplayWithoutMain(self)
    }

    pub fn display_link(&self) -> impl fmt::Display {
        DisplayLink(DisplayWithoutMain(self))
    }
}

impl FromStr for ArticleName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((group, page)) = s.split_once('/') {
            Ok(Self {
                group: group.parse()?,
                page: page.parse()?,
            })
        } else {
            Ok(Self {
                group: GroupName::default(),
                page: s.parse()?,
            })
        }
    }
}

impl fmt::Display for ArticleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.group, self.page)
    }
}

impl<'de> Deserialize<'de> for ArticleName {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        KString::deserialize(de)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

struct DisplayWithoutMain<'a>(&'a ArticleName);

impl fmt::Display for DisplayWithoutMain<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.group.is_main() {
            write!(f, "{}", self.0.page)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

struct DisplayLink<'a>(DisplayWithoutMain<'a>);

impl fmt::Display for DisplayLink<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.0;
        let url = name.0.url();
        write!(f, "\x1B]8;;{url}\x1B\\{name}\x1B]8;;\x1B\\")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn an(s: &str) -> ArticleName {
        s.parse().unwrap()
    }

    #[test]
    fn defaults_to_main_group() {
        assert_eq!(an("Foo").to_string(), "Main/Foo");
    }

    #[test]
    fn display_without_main_group() {
        assert_eq!(an("Main/Foo").display_without_main().to_string(), "Foo");
        assert_eq!(
            an("Other/Bar").display_without_main().to_string(),
            "Other/Bar"
        );
    }

    #[test]
    fn main_group_sorts_first() {
        let item = an("Foo");
        let before = an("A_Before/Bar");
        let after = an("Z_After/Baz");

        let mut items = vec![before.clone(), item.clone(), after.clone()];
        items.sort();

        assert_eq!(items, vec![item, before, after])
    }
}

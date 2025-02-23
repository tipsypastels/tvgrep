use kstring::KString;
use std::{convert::Infallible, fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArticleName {
    pub group: GroupName,
    pub page: PageName,
}

impl ArticleName {
    pub fn display_with_url(&self) -> ArticleNameDisplayWithUrl {
        ArticleNameDisplayWithUrl(self)
    }
}

impl FromStr for ArticleName {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Infallible> {
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

#[derive(Debug)]
pub struct ArticleNameDisplayWithUrl<'a>(&'a ArticleName);

impl fmt::Display for ArticleNameDisplayWithUrl<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.0;
        let url = article_url(name);
        write!(f, "\x1B]8;;{url}\x1B\\{name}\x1B]8;;\x1B\\",)
    }
}

// TODO: Should main sort first?
name_newtype!(GroupName);

impl Default for GroupName {
    fn default() -> Self {
        Self(KString::from_ref("Main"))
    }
}

name_newtype!(PageName);

macro_rules! name_newtype {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(KString);

        impl FromStr for $name {
            type Err = Infallible;

            fn from_str(s: &str) -> Result<Self, Infallible> {
                Ok(Self(KString::from_ref(s)))
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

use name_newtype;

use crate::url::article_url;

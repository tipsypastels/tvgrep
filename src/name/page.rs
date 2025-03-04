use anyhow::{Error, Result, ensure};
use kstring::KString;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PageName(KString);

impl FromStr for PageName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        ensure!(!s.is_empty(), "page name must not be empty");
        Ok(Self(KString::from_ref(s)))
    }
}

impl fmt::Display for PageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<str> for PageName {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

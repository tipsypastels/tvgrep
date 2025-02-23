use anyhow::{Error, Result, ensure};
use kstring::KString;
use std::{
    cmp::{Ordering, Reverse},
    fmt,
    str::FromStr,
};

pub const DEFAULT: &str = "Main";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupName(KString);

impl GroupName {
    pub fn is_default(&self) -> bool {
        self.0 == DEFAULT
    }
}

impl Default for GroupName {
    fn default() -> Self {
        Self(KString::from_static(DEFAULT))
    }
}

impl FromStr for GroupName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        ensure!(!s.is_empty(), "group name must not be empty");
        Ok(Self(KString::from_ref(s)))
    }
}

impl fmt::Display for GroupName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialOrd for GroupName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GroupName {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.is_default())
            .cmp(&Reverse(other.is_default()))
            .then(self.0.cmp(&other.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gn(s: &str) -> GroupName {
        s.parse().unwrap()
    }

    #[test]
    fn default_sorts_first() {
        let mut items: Vec<GroupName> = vec![gn("A_Before"), gn(DEFAULT), gn("Z_After")];
        items.sort();
        assert_eq!(items, vec![gn(DEFAULT), gn("A_Before"), gn("Z_After")]);
    }
}

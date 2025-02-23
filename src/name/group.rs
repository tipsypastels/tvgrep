use anyhow::{Error, Result, ensure};
use kstring::KString;
use serde::{Deserialize, Serialize};
use std::{
    cmp::{Ordering, Reverse},
    fmt,
    str::FromStr,
};

pub const MAIN: &str = "Main";

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct GroupName(KString);

impl GroupName {
    pub fn is_main(&self) -> bool {
        self.0 == MAIN
    }
}

impl Default for GroupName {
    fn default() -> Self {
        Self(KString::from_static(MAIN))
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
        Reverse(self.is_main())
            .cmp(&Reverse(other.is_main()))
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
    fn main_sorts_first() {
        let item = gn(MAIN);
        let before = gn("A_Before");
        let after = gn("Z_After");

        let mut items = vec![before.clone(), item.clone(), after.clone()];
        items.sort();

        assert_eq!(items, vec![item, before, after]);
    }
}

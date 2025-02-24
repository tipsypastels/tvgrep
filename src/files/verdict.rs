use anyhow::{Error, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Veredict {
    Yes,
    No,
}

impl FromStr for Veredict {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.eq_ignore_ascii_case("yes") {
            Ok(Self::Yes)
        } else if s.eq_ignore_ascii_case("no") {
            Ok(Self::No)
        } else {
            Err(anyhow!("unexpected verdict '{s}'"))
        }
    }
}

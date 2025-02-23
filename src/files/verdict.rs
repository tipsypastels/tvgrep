use anyhow::{Error, Result, bail};
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
        Ok(match s {
            "yes" => Self::Yes,
            "no" => Self::No,
            _ => bail!("unexpected verdict '{s}'"),
        })
    }
}

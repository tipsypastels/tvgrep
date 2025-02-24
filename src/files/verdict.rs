use anyhow::{Error, Result, anyhow};
use console::style;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Verdict {
    Yes,
    No,
}

impl FromStr for Verdict {
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

impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yes => write!(f, "{}", style("yes").green()),
            Self::No => write!(f, "{}", style("no").red()),
        }
    }
}

use anyhow::Result;
use kstring::KString;
use scraper::{Html, Selector};
use std::{fmt, sync::LazyLock};

static TROPE_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("li > a.twikilink:first-child").unwrap());

pub trait Query: fmt::Debug {
    type Output;
    fn query(&self, html: &Html) -> Result<Self::Output>;
}

#[derive(Debug)]
pub struct Stub;

impl Query for Stub {
    type Output = ();

    fn query(&self, _html: &Html) -> Result<Self::Output> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct FlatList;

impl Query for FlatList {
    type Output = Vec<KString>;

    fn query(&self, html: &Html) -> Result<Self::Output> {
        let tropes = html.select(&TROPE_SEL);
        let mut out = Vec::new();

        for trope in tropes {
            out.push(KString::from_string(trope.text().collect::<String>()))
        }

        Ok(out)
    }
}

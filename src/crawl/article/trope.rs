use anyhow::Result;
use kstring::KString;
use scraper::{Html, Selector};
use std::sync::LazyLock;

static TROPE_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("li > a.twikilink:first-child").unwrap());

pub trait TropeQuery {
    type Output;
    fn query(html: &Html) -> Result<Self::Output>;
}

#[derive(Debug)]
pub struct TropeQueryIgnore;

impl TropeQuery for TropeQueryIgnore {
    type Output = ();

    fn query(_html: &Html) -> Result<Self::Output> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct TropeQueryFlatList;

impl TropeQuery for TropeQueryFlatList {
    type Output = Vec<KString>;

    fn query(html: &Html) -> Result<Self::Output> {
        let tropes = html.select(&TROPE_SEL);
        let mut out = Vec::new();

        for trope in tropes {
            out.push(KString::from_string(trope.text().collect::<String>()))
        }

        Ok(out)
    }
}

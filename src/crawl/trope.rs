use crate::{data, name::ArticleName};
use anyhow::{Context, Result};
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
    type Output = data::TropeDataStub;

    fn query(&self, _html: &Html) -> Result<Self::Output> {
        Ok(data::TropeDataStub)
    }
}

#[derive(Debug)]
pub struct FlatList;

impl Query for FlatList {
    type Output = data::TropeDataFlatList;

    fn query(&self, html: &Html) -> Result<Self::Output> {
        let tropes = html.select(&TROPE_SEL);
        let mut out = Vec::new();

        for trope in tropes {
            let url = trope.attr("href").context("link has no url")?;
            let article = ArticleName::from_url(url)?;
            out.push(article);
        }

        Ok(data::TropeDataFlatList(out))
    }
}

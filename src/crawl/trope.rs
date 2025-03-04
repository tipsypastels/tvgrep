use crate::{data, name::ArticleName};
use anyhow::{Context, Result};
use kstring::KString;
use scraper::{ElementRef, Html, Selector};
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
pub struct Single<'a>(pub &'a ArticleName);

impl Query for Single<'_> {
    type Output = data::TropeDataSingle;

    fn query(&self, html: &Html) -> Result<Self::Output> {
        let trope = self.0.clone();

        let Some(trope_node) = html.select(&TROPE_SEL).find(|node| {
            node.attr("href")
                .is_some_and(|url| self.0.matches_relative_url(url))
        }) else {
            return Ok(data::TropeDataSingle::missing(trope));
        };

        let li_node = trope_node.parent().context("trope_node lacks parent")?;
        let li_node = ElementRef::wrap(li_node).context("trope_node lacks parent")?;
        let li_node_text = li_node
            .text()
            .filter(|s| !s.trim().is_empty())
            .enumerate()
            .map(|(i, s)| {
                if i == 1 {
                    s.strip_prefix(": ").unwrap_or(s)
                } else {
                    s
                }
            })
            .skip(1) // skip trope name
            .collect::<String>();

        let text = (!li_node_text.is_empty()).then(|| KString::from_string(li_node_text));
        Ok(data::TropeDataSingle::new(trope, text))
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

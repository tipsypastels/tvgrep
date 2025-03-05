use crate::{
    data::{ArticleData, TropeDataSingle},
    name::{ArticleName, GroupName},
};
use anyhow::Result;
use futures::{Stream, StreamExt};
use std::{
    fmt::{self, Display},
    pin::pin,
};

#[allow(unused)]
pub fn print<I, P>(unfiltered_len: Option<usize>, iter: I)
where
    I: Iterator<Item = P>,
    P: PrintArticleEntry,
{
    let mut state = PrinterState::new();
    for entry in iter {
        state.advance(&entry);
    }
    state.finish(unfiltered_len);
}

pub async fn print_async<S, P>(unfiltered_len: Option<usize>, stream: S)
where
    S: Stream<Item = P>,
    P: PrintArticleEntry,
{
    let mut state = PrinterState::new();
    let mut stream = pin!(stream);
    while let Some(entry) = stream.next().await {
        state.advance(&entry);
    }
    state.finish(unfiltered_len);
}

struct PrinterState {
    group: Option<GroupName>,
    count: usize,
}

impl PrinterState {
    fn new() -> Self {
        Self {
            group: None,
            count: 0,
        }
    }

    fn advance<P: PrintArticleEntry>(&mut self, entry: &P) {
        self.count += 1;

        let group = entry.group();
        if self.group.is_none() || self.group.as_ref().is_some_and(|g| g != group) {
            self.group = Some(group.clone());

            println!();
            println!("{group}");
            println!();
        }

        println!("\t{}", entry.display());
    }

    fn finish(self, unfiltered_len: Option<usize>) {
        println!();
        println!("({} results)", self.count);

        if let Some(unfiltered_len) = unfiltered_len {
            let filtered_out = unfiltered_len - self.count;
            println!("({filtered_out} results filtered out)");
        }
    }
}

pub trait PrintArticleEntry {
    fn group(&self) -> &GroupName;
    fn display(&self) -> impl Display;
}

impl PrintArticleEntry for ArticleName {
    fn group(&self) -> &GroupName {
        &self.group
    }

    fn display(&self) -> impl Display {
        self.display_link()
    }
}

#[derive(Debug)]
pub struct ArticleAndTropeDesc<'a> {
    name: &'a ArticleName,
    data: Result<ArticleData<TropeDataSingle>>,
}

impl<'a> ArticleAndTropeDesc<'a> {
    pub fn new(name: &'a ArticleName, data: Result<ArticleData<TropeDataSingle>>) -> Self {
        Self { name, data }
    }
}

impl PrintArticleEntry for ArticleAndTropeDesc<'_> {
    fn group(&self) -> &GroupName {
        &self.name.group
    }

    fn display(&self) -> impl Display {
        self
    }
}

impl Display for ArticleAndTropeDesc<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name.display_link())?;
        match &self.data {
            Ok(data) => writeln!(f, "{}", data.tropes.display_text()),
            Err(_) => writeln!(f, "{}", console::style("(load failed)").on_red()),
        }
    }
}

impl<P: PrintArticleEntry> PrintArticleEntry for &P {
    fn group(&self) -> &GroupName {
        (*self).group()
    }

    fn display(&self) -> impl Display {
        (*self).display()
    }
}

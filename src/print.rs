use crate::name::{ArticleName, GroupName};
use futures::{Stream, StreamExt};
use std::{
    fmt::{self, Display},
    pin::pin,
};

#[derive(Debug, Clone)]
pub struct Printer<I> {
    iter: I,
    unfiltered_len: Option<usize>,
}

impl<I> Printer<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            unfiltered_len: None,
        }
    }

    pub fn unfiltered_len(mut self, len: usize) -> Self {
        self.unfiltered_len = Some(len);
        self
    }
}

impl<'a, I, P> Printer<I>
where
    I: Iterator<Item = &'a P>,
    P: PrintArticleEntry + 'a,
{
    #[allow(unused)]
    pub fn print(self) {
        let mut state = PrinterState::new();

        for entry in self.iter {
            state.advance(entry);
        }

        state.finish(self.unfiltered_len);
    }
}

impl<'a, S, P> Printer<S>
where
    S: Stream<Item = &'a P>,
    P: PrintArticleEntry + 'a,
{
    pub async fn print_async(self) {
        let mut state = PrinterState::new();
        let mut stream = pin!(self.iter);

        while let Some(entry) = stream.next().await {
            state.advance(entry);
        }

        state.finish(self.unfiltered_len);
    }
}

struct PrinterState<'a> {
    group: Option<&'a GroupName>,
    count: usize,
}

impl<'a> PrinterState<'a> {
    fn new() -> Self {
        Self {
            group: None,
            count: 0,
        }
    }

    fn advance<P: PrintArticleEntry>(&mut self, entry: &'a P) {
        self.count += 1;

        let group = entry.group();
        if self.group.is_none() || self.group.is_some_and(|g| g != group) {
            self.group = Some(group);

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

// impl<T> PrintArticleEntry for (ArticleName, T)
// where
//     T: Display,
// {
//     fn group(&self) -> &GroupName {
//         &self.0.group
//     }

//     fn display(&self) -> impl Display {
//         struct TupleDisplay<'a, T>(&'a ArticleName, &'a T);
//         impl<T> Display for TupleDisplay<'_, T>
//         where
//             T: Display,
//         {
//             fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//                 write!(f, "{} ({})", self.0.display_link(), self.1)
//             }
//         }
//         TupleDisplay(&self.0, &self.1)
//     }
// }

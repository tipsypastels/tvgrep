use crate::name::{ArticleName, GroupName};
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct Printer<I> {
    iter: I,
    unfiltered_len: Option<usize>,
}

impl<'a, I, P> Printer<I>
where
    I: Iterator<Item = &'a P>,
    P: PrintableArticleEntry + 'a,
{
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

    pub fn print(self) {
        let mut cur_group = None;
        let mut cnt = 0usize;

        for entry in self.iter {
            cnt += 1;

            let group = entry.group();

            if cur_group.is_none() || cur_group.is_some_and(|cg| cg != group) {
                cur_group = Some(group);

                println!();
                println!("{group}");
                println!();
            }

            println!("\t{}", entry.display());
        }

        println!();
        println!("({cnt} results)");

        if let Some(unfiltered_len) = self.unfiltered_len {
            let filtered_out = unfiltered_len - cnt;
            println!("({filtered_out} results filtered out)")
        }
    }
}

pub trait PrintableArticleEntry {
    fn group(&self) -> &GroupName;
    fn display(&self) -> impl Display;
}

impl PrintableArticleEntry for ArticleName {
    fn group(&self) -> &GroupName {
        &self.group
    }

    fn display(&self) -> impl Display {
        self.display_link()
    }
}

impl<T> PrintableArticleEntry for (ArticleName, T)
where
    T: Display,
{
    fn group(&self) -> &GroupName {
        &self.0.group
    }

    fn display(&self) -> impl Display {
        struct TupleDisplay<'a, T>(&'a ArticleName, &'a T);
        impl<T> Display for TupleDisplay<'_, T>
        where
            T: Display,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{} ({})", self.0, self.1)
            }
        }
        TupleDisplay(&self.0, &self.1)
    }
}

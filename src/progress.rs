use std::fmt::{self, Display};

#[derive(Debug, Copy, Clone)]
pub struct Progress {
    cur: usize,
    max: Option<usize>,
}

impl Display for Progress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(max) = self.max {
            write!(f, "({}/{max})", self.cur + 1)
        } else {
            write!(f, "({}/?)", self.cur + 1)
        }
    }
}

pub trait WithProgress<T> {
    fn with_progress(self) -> impl Iterator<Item = (Progress, T)>;
}

impl<I, T> WithProgress<T> for I
where
    I: Iterator<Item = T>,
{
    fn with_progress(self) -> impl Iterator<Item = (Progress, T)> {
        let max = self.size_hint().1;
        self.enumerate()
            .map(move |(cur, item)| (Progress { cur, max }, item))
    }
}

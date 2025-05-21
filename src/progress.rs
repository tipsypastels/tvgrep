use std::fmt::{self, Display};

#[derive(Debug, Copy, Clone)]
pub struct Progress {
    cur: usize,
    max: Option<usize>,
}

impl Progress {
    pub fn new(cur: usize, max: Option<usize>) -> Self {
        Self { cur, max }
    }
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

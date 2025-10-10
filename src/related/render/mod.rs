mod entries;
mod modal;

use super::list::RelatedArticleEntry;
use crate::{name::ArticleName, render::error};
use anyhow::Error;
use ratatui::{prelude::*, widgets::ListState};

pub struct RelatedRenderer<'a> {
    pub article_name: &'a ArticleName,
    pub list_state: &'a mut ListState,
    pub list_entries: &'a [RelatedArticleEntry],
    pub modal: Option<&'a mut RelatedModal>,
    pub error: Option<&'a Error>,
}

pub enum RelatedModal {
    SetVerdict {
        article_name: ArticleName,
        list_state: ListState,
    },
    SetGroup {
        buffer: String,
    },
}

pub fn main(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    let cols = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .spacing(1)
        .split(area);

    entries::main(re, cols[1], buf);
    modal::main(re, area, buf);

    if let Some(error) = re.error {
        error::error(error, area, buf);
    }
}

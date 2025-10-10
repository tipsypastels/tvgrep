mod entries;
mod modal;
mod status;

use super::list::RelatedArticleEntry;
use crate::{app::RenderInfo, name::ArticleName, render::error};
use ratatui::{prelude::*, widgets::ListState};

pub struct RelatedRenderer<'a> {
    pub article_name: &'a ArticleName,
    pub list_state: &'a mut ListState,
    pub list_entries: &'a [RelatedArticleEntry],
    pub modal: Option<&'a mut RelatedModal>,
    pub info: RenderInfo<'a>,
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
    let rows =
        Layout::vertical([Constraint::Percentage(85), Constraint::Percentage(15)]).split(area);

    let cols = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .spacing(1)
        .split(rows[0]);

    entries::main(re, cols[1], buf);
    modal::main(re, area, buf);
    status::main(re, rows[1], buf);

    if re.info.loading {
        if re.info.quitting {
            modal::waiting_to_quit(area, buf);
        } else if re.list_entries.is_empty() {
            modal::loading_initial(area, buf);
        }
    }

    if let Some(error) = re.info.error {
        error::error(error, area, buf);
    }
}

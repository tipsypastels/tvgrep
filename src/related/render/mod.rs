mod entries;
mod modal;

use super::RelatedRenderer;
use ratatui::{prelude::*, widgets::Paragraph};

pub fn main(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    // TODO
    if re.list_entries.is_empty() && re.list_loading {
        Paragraph::new("Loading...").render(area, buf);
        return;
    }

    let cols = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .spacing(1)
        .split(area);

    entries::main(re, cols[1], buf);
    modal::main(re, area, buf);
}

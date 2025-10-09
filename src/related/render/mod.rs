mod entries;
mod modal;

use super::RelatedRenderer;
use ratatui::prelude::*;

pub fn main(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    let cols = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .spacing(1)
        .split(area);

    entries::main(re, cols[1], buf);
    modal::main(re, area, buf);
}

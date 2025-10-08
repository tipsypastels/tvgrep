use super::RelatedRenderer;
use ratatui::{prelude::*, widgets::Block};

pub fn main(re: RelatedRenderer, area: Rect, buf: &mut Buffer) {
    Block::bordered()
        .title(Line::styled(format!(" Related: {} ", re.name), Modifier::BOLD).centered())
        .render(area, buf);
}

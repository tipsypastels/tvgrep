use super::RelatedRenderer;
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

pub fn main(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    let mut spans = Vec::new();

    if re.info.loading {
        const SYMBOLS: [&str; 4] = ["◐", "◓", "◑", "◒"];

        let symbol = SYMBOLS[re.info.frame_no % 4];
        let message = format!("{symbol} Working...");

        spans.push(Span::raw(message));
    }

    Paragraph::new(Line::from(spans))
        .centered()
        .block(Block::bordered())
        .render(area, buf);
}

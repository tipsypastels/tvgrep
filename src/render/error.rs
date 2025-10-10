use anyhow::Error;
use ratatui::{
    prelude::*,
    widgets::{Block, Clear, Padding, Paragraph},
};

const SCREEN_PERCENT: (u16, u16) = (33, 40);

pub fn error(error: &Error, area: Rect, buf: &mut Buffer) {
    let text = format!("{error:?}");
    let area = split_area(area);
    let block = Block::bordered()
        .title(" Error ")
        .padding(Padding::symmetric(1, 0))
        .style(Style::new().on_light_red())
        .border_style(Style::new().black());

    Clear.render(area, buf);
    Paragraph::new(text).block(block).render(area, buf);
}

fn split_area(area: Rect) -> Rect {
    let (percent_x, percent_y) = SCREEN_PERCENT;
    let vert = Layout::vertical([
        Constraint::Percentage(100 - percent_y),
        Constraint::Percentage(percent_y),
    ]);
    let horiz = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ]);
    horiz.split(vert.split(area)[1])[1]
}

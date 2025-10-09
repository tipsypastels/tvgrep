use ratatui::{
    prelude::*,
    widgets::{Block, Clear},
};
use std::borrow::Cow;

const DEFAULT_SCREEN_PERCENT: (u16, u16) = (60, 31);

pub struct Modal<'a> {
    area: Rect,
    buf: &'a mut Buffer,
    title: Option<Cow<'a, str>>,
    title_bottom: Option<Cow<'a, str>>,
    screen_percent: (u16, u16),
}

impl<'a> Modal<'a> {
    pub fn new(area: Rect, buf: &'a mut Buffer) -> Self {
        Self {
            area,
            buf,
            title: None,
            title_bottom: None,
            screen_percent: DEFAULT_SCREEN_PERCENT,
        }
    }

    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn title_bottom(mut self, title_bottom: impl Into<Cow<'a, str>>) -> Self {
        self.title_bottom = Some(title_bottom.into());
        self
    }

    pub fn screen_percent(mut self, x: u16, y: u16) -> Self {
        self.screen_percent = (x, y);
        self
    }

    pub fn render(self, f: impl FnOnce(Rect, &mut Buffer, Block)) {
        let area = self.split_area();
        let block = Block::bordered()
            .style(Style::new().white().on_black())
            .border_style(Style::new().white());

        let block = if let Some(title) = self.title {
            block.title(Line::styled(title, Modifier::BOLD))
        } else {
            block
        };

        let block = if let Some(title_bottom) = self.title_bottom {
            block.title_bottom(Line::styled(title_bottom, Modifier::BOLD).right_aligned())
        } else {
            block
        };

        Clear.render(area, self.buf);
        f(area, self.buf, block)
    }

    fn split_area(&self) -> Rect {
        let (percent_x, percent_y) = self.screen_percent;
        let vert = Layout::vertical([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ]);
        let horiz = Layout::horizontal([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ]);
        horiz.split(vert.split(self.area)[1])[1]
    }
}

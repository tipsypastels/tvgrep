use super::RelatedRenderer;
use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Paragraph, Wrap},
};

pub fn main(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    let Some(article_info) = re.article_info.as_mut() else {
        return;
    };

    let block = Block::bordered()
        .title(
            Line::styled(
                format!(" Viewing: {} ", article_info.article_name()),
                Modifier::BOLD,
            )
            .centered(),
        )
        .title_bottom(Line::styled(" Open <O> ", Modifier::BOLD).centered())
        .padding(Padding::uniform(1));

    Paragraph::new(vec![
        Line::styled(article_info.title(), Modifier::BOLD),
        Line::raw(""),
        Line::raw("---"),
        Line::raw(""),
        Line::raw(article_info.summary()),
    ])
    .wrap(Wrap { trim: false })
    .block(block)
    .render(area, buf);
}

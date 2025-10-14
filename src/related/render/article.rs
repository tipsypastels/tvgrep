use super::RelatedRenderer;
use crate::related::article::RelatedArticleInfoTab;
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
        .title_bottom(Line::styled(" Toggle <T> Open <O> ", Modifier::BOLD).centered())
        .padding(Padding::uniform(1));

    let content_line = match article_info.tab() {
        RelatedArticleInfoTab::Summary => Line::raw(article_info.summary()),
        RelatedArticleInfoTab::Trope => {
            let trope = article_info.trope();
            Line::from(vec![
                Span::styled(
                    format!("{}: ", trope.article_name.display_without_main()),
                    Modifier::BOLD,
                ),
                Span::raw(trope.text.as_ref().map(|t| t.as_str()).unwrap_or("no text")),
            ])
        }
    };

    Paragraph::new(vec![
        Line::styled(article_info.title(), Modifier::BOLD),
        Line::raw(""),
        Line::raw("---"),
        Line::raw(""),
        content_line,
    ])
    .wrap(Wrap { trim: false })
    .block(block)
    .render(area, buf);
}

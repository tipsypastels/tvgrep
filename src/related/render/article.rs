use super::RelatedRenderer;
use crate::{crawl::article::ArticleSingleTropeBody, related::article::RelatedArticleInfoTab};
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

    let mut lines = vec![
        Line::styled(article_info.title(), Modifier::BOLD),
        Line::raw(""),
        Line::raw("---"),
        Line::raw(""),
    ];

    match article_info.tab() {
        RelatedArticleInfoTab::Summary => {
            lines.push(Line::raw(article_info.summary()));
        }
        RelatedArticleInfoTab::Trope => match article_info.trope() {
            ArticleSingleTropeBody::TopLevel { article_name, text } => {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("{}: ", article_name.display_without_main()),
                        Modifier::BOLD,
                    ),
                    Span::raw(text.as_str()),
                ]));
            }
            ArticleSingleTropeBody::InOther {
                other_article_name,
                text,
                own_article_url_range: Some(own_article_url_range),
            } => {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("{}: ", other_article_name.display_without_main()),
                        Modifier::BOLD,
                    ),
                    Span::raw(&text[..own_article_url_range.start]),
                    Span::styled(&text[own_article_url_range.clone()], Modifier::UNDERLINED),
                    Span::raw(&text[own_article_url_range.end..]),
                ]));
            }
            ArticleSingleTropeBody::InOther {
                other_article_name,
                text,
                own_article_url_range: None,
            } => {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("{}: ", other_article_name.display_without_main()),
                        Modifier::BOLD,
                    ),
                    Span::raw(text.as_str()),
                ]));
            }
            ArticleSingleTropeBody::Elsewhere {
                nearest_block_parent_text,
                url_range: Some(url_range),
            } => {
                lines.extend([
                    Line::styled(
                        "(found elsewhere)",
                        Style::new().italic().fg(Color::DarkGray),
                    ),
                    Line::from(vec![
                        Span::raw(&nearest_block_parent_text[..url_range.start]),
                        Span::styled(
                            &nearest_block_parent_text[url_range.clone()],
                            Modifier::UNDERLINED,
                        ),
                        Span::raw(&nearest_block_parent_text[url_range.end..]),
                    ]),
                ]);
            }
            ArticleSingleTropeBody::Elsewhere {
                nearest_block_parent_text,
                url_range: None,
            } => {
                lines.extend([
                    Line::styled(
                        "(found elsewhere)",
                        Style::new().italic().fg(Color::DarkGray),
                    ),
                    Line::raw(nearest_block_parent_text.as_str()),
                ]);
            }
        },
    };

    Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(block)
        .render(area, buf);
}

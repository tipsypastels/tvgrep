use super::RelatedRenderer;
use crate::database::Verdict;
use ratatui::{
    prelude::*,
    widgets::{Block, List, Padding},
};

pub fn main(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    let entries = re.list.iter_with_selected().map(|(entry, selected)| {
        let text = entry.article_name.display_without_main().to_string();
        let color = entry.verdict.map(verdict_color);
        let style = if selected {
            Style::new().black().bg(color.unwrap_or(Color::White))
        } else {
            Style::new().fg(color.unwrap_or(Color::default()))
        };

        Text::styled(text, style)
    });

    let block = Block::bordered()
        .title(Line::styled(format!(" Related: {} ", re.article_name), Modifier::BOLD).centered())
        .title_bottom(
            Line::styled(
                " Set Verdict <W> Filter Verdict <F> Filter Group <G> ",
                Modifier::BOLD,
            )
            .centered(),
        )
        .padding(Padding::uniform(1));

    StatefulWidget::render(List::new(entries).block(block), area, buf, re.list.state());
}

fn verdict_color(verdict: Verdict) -> Color {
    match verdict {
        Verdict::Yes => Color::Green,
        Verdict::No => Color::Red,
        Verdict::Ignore => Color::DarkGray,
    }
}

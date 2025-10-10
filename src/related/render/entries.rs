use super::RelatedRenderer;
use crate::database::Verdict;
use ratatui::{
    prelude::*,
    widgets::{Block, List, Padding, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

pub fn main(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    let entries = re
        .list
        .iter_with_selected()
        .map(|(entry, selected)| {
            let text = entry.article_name.display_without_main().to_string();
            let color = entry.verdict.map(verdict_color);
            let style = if selected {
                Style::new().black().bg(color.unwrap_or(Color::White))
            } else {
                Style::new().fg(color.unwrap_or(Color::default()))
            };

            Text::styled(text, style)
        })
        .chain((!re.list.exhausted() && !re.info.loading).then(|| {
            Text::styled(
                "...Load More...",
                if re.list.selected_load_more() {
                    Style::new().black().on_white()
                } else {
                    Style::new().bold()
                },
            )
        }));

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

    let scrollbar_area = block.inner(area);
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    let mut scrollbar_state = ScrollbarState::new(re.list.len_including_virtual())
        .position(re.list.selected_idx().unwrap_or_default());

    StatefulWidget::render(List::new(entries).block(block), area, buf, re.list.state());
    StatefulWidget::render(scrollbar, scrollbar_area, buf, &mut scrollbar_state);
}

fn verdict_color(verdict: Verdict) -> Color {
    match verdict {
        Verdict::Yes => Color::Green,
        Verdict::No => Color::Red,
        Verdict::Ignore => Color::DarkGray,
    }
}

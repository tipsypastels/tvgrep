use crate::related::list::VerdictFilter;

use super::RelatedRenderer;
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

pub fn main(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    let mut spans = Vec::new();

    macro_rules! push {
        ($($spans:expr),*$(,)?) => {
            spans.extend([$($spans,)* Span::raw("  ")]);
        };
    }

    if re.info.loading {
        const SYMBOLS: [&str; 4] = ["◐", "◓", "◑", "◒"];
        push!(
            Span::raw(SYMBOLS[re.info.frame_no % 4]),
            Span::styled(" Working...", Modifier::BOLD)
        );
    }

    match re.list.verdict() {
        VerdictFilter::Eq(verdict) => {
            push!(
                Span::styled("Verdict: ", Modifier::BOLD),
                Span::raw(verdict.name())
            );
        }
        VerdictFilter::Unset => {
            push!(
                Span::styled("Verdict: ", Modifier::BOLD),
                Span::raw("Unset")
            );
        }
        VerdictFilter::None => {}
    }

    if let Some(group_name) = re.list.group_name() {
        push!(
            Span::styled("Group: ", Modifier::BOLD),
            Span::raw(group_name.to_string())
        );
    }

    Paragraph::new(Line::from(spans))
        .centered()
        .block(Block::bordered())
        .render(area, buf);
}

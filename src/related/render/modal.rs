use crate::{
    database::Verdict,
    related::{RelatedModal, RelatedRenderer},
    render::modal::Modal,
};
use ratatui::{prelude::*, widgets::List};

pub fn main(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    match re.modal {
        Some(RelatedModal::SetVerdict { .. }) => set_verdict(re, area, buf),
        None => {}
    }
}

fn set_verdict(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    let (name, list_state) = match re.modal {
        Some(RelatedModal::SetVerdict { name, list_state }) => (name, list_state),
        _ => unreachable!(),
    };

    Modal::new(area, buf)
        .title(format!(" Verdict for {name} "))
        .title_bottom(" Choose <Enter> Close <Esc> ")
        .render(|area, buf, block| {
            let entries = Verdict::variants()
                .map(|v| v.name())
                .chain(std::iter::once("Unset"))
                .enumerate()
                .map(|(i, s)| format!("{}. {s}", i + 1));

            StatefulWidget::render(
                List::new(entries)
                    .highlight_style(Style::new().black().on_white())
                    .block(block),
                area,
                buf,
                list_state,
            );
        });
}

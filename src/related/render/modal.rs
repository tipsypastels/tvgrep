use super::{RelatedModal, RelatedRenderer};
use crate::{database::Verdict, render::modal::Modal};
use ratatui::{
    prelude::*,
    widgets::{List, Paragraph},
};

pub fn main(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    match re.modal {
        Some(RelatedModal::SetVerdict { .. }) => set_verdict(re, area, buf),
        Some(RelatedModal::SetGroup { .. }) => set_group(re, area, buf),
        None => {}
    }
}

fn set_verdict(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    let (name, list_state) = match re.modal {
        Some(RelatedModal::SetVerdict {
            article_name: name,
            list_state,
        }) => (name, list_state),
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

fn set_group(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    let buffer = match re.modal {
        Some(RelatedModal::SetGroup { buffer }) => buffer,
        _ => unreachable!(),
    };

    Modal::new(area, buf)
        .title(" Filter Search ")
        .title_bottom(" Submit <Enter> Close <Esc> ")
        .screen_percent(40, 20)
        .render(|area, buf, block| {
            Paragraph::new(&**buffer).block(block).render(area, buf);
        });
}

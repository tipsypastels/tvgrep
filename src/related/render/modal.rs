use super::{RelatedModal, RelatedRenderer};
use crate::{database::Verdict, render::modal::Modal};
use ratatui::{
    prelude::*,
    widgets::{List, ListState, Paragraph},
};

pub fn main(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    match re.modal {
        Some(RelatedModal::SetVerdict { .. }) => set_verdict(re, area, buf),
        Some(RelatedModal::FilterVerdict { .. }) => filter_verdict(re, area, buf),
        Some(RelatedModal::FilterGroup { .. }) => filter_group(re, area, buf),
        None => {}
    }
}

pub fn loading_initial(area: Rect, buf: &mut Buffer) {
    Modal::new(area, buf)
        .title(" Loading Related... ")
        .screen_percent(30, 18)
        .render(|area, buf, block| {
            Paragraph::new("...")
                .centered()
                .block(block)
                .render(area, buf);
        });
}

pub fn waiting_to_quit(area: Rect, buf: &mut Buffer) {
    Modal::new(area, buf)
        .title(" Shutting Down ")
        .screen_percent(30, 18)
        .render(|area, buf, block| {
            Paragraph::new("Waiting for jobs to finish...")
                .centered()
                .block(block)
                .render(area, buf);
        });
}

fn set_verdict(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    let (name, list_state) = match re.modal {
        Some(RelatedModal::SetVerdict {
            article_name: name,
            list_state,
        }) => (name, list_state),
        _ => unreachable!(),
    };

    let modal = Modal::new(area, buf).title(format!(" Set Verdict for {name} "));

    render_verdict_options_modal(modal, ["Unset"], list_state);
}

fn filter_verdict(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    let list_state = match re.modal {
        Some(RelatedModal::FilterVerdict { list_state }) => list_state,
        _ => unreachable!(),
    };

    let modal = Modal::new(area, buf)
        .title(" Filter Verdict ")
        .screen_percent_y(41);

    render_verdict_options_modal(modal, ["Unset", "None"], list_state);
}

fn render_verdict_options_modal(
    modal: Modal,
    extras: impl IntoIterator<Item = &'static str>,
    list_state: &mut ListState,
) {
    modal
        .title_bottom(" Choose <Enter> Close <Esc> ")
        .render(|area, buf, block| {
            let entries = Verdict::variants()
                .map(|v| v.name())
                .chain(extras)
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

fn filter_group(re: &mut RelatedRenderer, area: Rect, buf: &mut Buffer) {
    let buffer = match re.modal {
        Some(RelatedModal::FilterGroup { buffer }) => buffer,
        _ => unreachable!(),
    };

    Modal::new(area, buf)
        .title(" Filter Group ")
        .title_bottom(" Submit <Enter> Close <Esc> ")
        .screen_percent(40, 20)
        .render(|area, buf, block| {
            Paragraph::new(&**buffer).block(block).render(area, buf);
        });
}

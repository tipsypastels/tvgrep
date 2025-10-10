use super::RelatedMessage;
use crate::{
    database::Verdict,
    name::{ArticleName, GroupName},
    render::list::ListStateExt,
};
use ratatui::widgets::ListState;

pub struct RelatedArticleList {
    state: ListState,
    entries: Vec<RelatedArticleEntry>,
    group_name: Option<GroupName>,
    page: u8,
    verdict: VerdictFilter,
    needs_reload: bool,
    never_loaded_any: bool,
}

pub struct RelatedArticleEntry {
    pub article_name: ArticleName,
    pub verdict: Option<Verdict>,
}

impl RelatedArticleList {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
            entries: Vec::new(),
            group_name: None,
            page: 1,
            verdict: VerdictFilter::None,
            needs_reload: false,
            never_loaded_any: true,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &RelatedArticleEntry> {
        self.entries
            .iter()
            .filter(|entry| self.verdict.matches(entry.verdict))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut RelatedArticleEntry> {
        self.entries
            .iter_mut()
            .filter(|entry| self.verdict.matches(entry.verdict))
    }

    pub fn iter_with_selected(&self) -> impl Iterator<Item = (&RelatedArticleEntry, bool)> {
        self.iter()
            .enumerate()
            .map(|(i, e)| (e, self.state.selected().is_some_and(|s| s == i)))
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn selected(&self) -> Option<&RelatedArticleEntry> {
        self.state.selected().and_then(|i| self.iter().nth(i))
    }

    pub fn selected_mut(&mut self) -> Option<&mut RelatedArticleEntry> {
        self.state.selected().and_then(|i| self.iter_mut().nth(i))
    }

    pub fn state(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn select_prev_or_last(&mut self) {
        self.state.select_prev_or_last();
    }

    pub fn select_next_or_first(&mut self) {
        self.state.select_next_or_first(self.len());
    }

    pub fn group_name(&self) -> Option<&GroupName> {
        self.group_name.as_ref()
    }

    pub fn verdict(&self) -> VerdictFilter {
        self.verdict
    }

    pub fn never_loaded_any(&self) -> bool {
        self.never_loaded_any
    }

    pub fn load(&self) -> RelatedMessage {
        RelatedMessage::LoadRelated {
            group_name: self.group_name.clone(),
            page: self.page,
        }
    }

    pub fn loaded(&mut self, entries: Vec<RelatedArticleEntry>) {
        if self.needs_reload {
            self.entries = entries;
            self.needs_reload = false;
        } else {
            self.entries.extend(entries);
        }
        self.never_loaded_any = false;
    }

    pub fn set_group_name(&mut self, group_name: Option<GroupName>) -> RelatedMessage {
        self.group_name = group_name;
        self.needs_reload = true;
        self.state.select(None);
        self.load()
    }

    pub fn set_verdict(&mut self, verdict: VerdictFilter) {
        self.verdict = verdict;
        self.state.select(None);
    }
}

#[derive(Copy, Clone)]
pub enum VerdictFilter {
    Eq(Verdict),
    Unset,
    None,
}

impl VerdictFilter {
    pub fn matches(self, verdict: Option<Verdict>) -> bool {
        match self {
            Self::Eq(v) => verdict.is_some_and(|ev| ev == v),
            Self::Unset => verdict.is_none(),
            Self::None => true,
        }
    }
}

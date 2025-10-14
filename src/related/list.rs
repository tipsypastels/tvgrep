use self::computed_len::ComputedLen;
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
    len: ComputedLen,
    group_name: Option<GroupName>,
    page: u8,
    exhausted: bool,
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
            len: ComputedLen::new(),
            group_name: None,
            page: 1,
            exhausted: false,
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
        self.len.into_inner()
    }

    pub fn len_including_virtual(&self) -> usize {
        self.len() + if self.exhausted { 0 } else { 1 }
    }

    pub fn exhausted(&self) -> bool {
        self.exhausted
    }

    pub fn selected(&self) -> Option<&RelatedArticleEntry> {
        self.state.selected().and_then(|i| self.iter().nth(i))
    }

    pub fn selected_mut(&mut self) -> Option<&mut RelatedArticleEntry> {
        self.state.selected().and_then(|i| self.iter_mut().nth(i))
    }

    pub fn selected_idx(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn selected_load_more(&self) -> bool {
        !self.exhausted && self.state.selected().is_some_and(|i| i == self.len())
    }

    pub fn state(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn select_first(&mut self) {
        self.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.state.select_last();
    }

    pub fn select_prev_or_last(&mut self) {
        self.state.select_prev_or_last();
    }

    pub fn select_next_or_first(&mut self) {
        self.state
            .select_next_or_first(self.len_including_virtual());
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
            self.exhausted = entries.is_empty();
            self.entries.extend(entries);
            self.page += 1;
        }
        self.recalc_len();
        self.never_loaded_any = false;
    }

    pub fn set_group_name(&mut self, group_name: Option<GroupName>) -> RelatedMessage {
        self.group_name = group_name;
        self.page = 1;
        self.exhausted = false;
        self.needs_reload = true;
        self.state.select(None);
        self.load()
    }

    pub fn set_verdict(&mut self, verdict: VerdictFilter) {
        self.verdict = verdict;
        self.state.select(None);
        self.recalc_len();
    }

    fn recalc_len(&mut self) {
        let count = self.iter().count();
        let promise = computed_len::IPromiseIAmSettingThisFromIterCount;
        self.len.set(count, promise);
    }
}

mod computed_len {
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ComputedLen(usize);

    pub struct IPromiseIAmSettingThisFromIterCount;

    impl ComputedLen {
        pub fn new() -> Self {
            Self(0)
        }

        pub fn into_inner(self) -> usize {
            self.0
        }

        pub fn set(&mut self, count: usize, _: IPromiseIAmSettingThisFromIterCount) {
            self.0 = count;
        }
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

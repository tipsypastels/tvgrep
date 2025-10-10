use super::RelatedMessage;
use crate::{
    database::Verdict,
    name::{ArticleName, GroupName},
};

pub struct RelatedArticleList {
    entries: Vec<RelatedArticleEntry>,
    group_name: Option<GroupName>,
    page: u8,
    filter_dirty: bool,
}

pub struct RelatedArticleEntry {
    pub article_name: ArticleName,
    pub verdict: Option<Verdict>,
}

impl RelatedArticleList {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            group_name: None,
            page: 1,
            filter_dirty: false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> &[RelatedArticleEntry] {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut [RelatedArticleEntry] {
        &mut self.entries
    }

    pub fn get(&self, i: usize) -> Option<&RelatedArticleEntry> {
        self.entries.get(i)
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut RelatedArticleEntry> {
        self.entries.get_mut(i)
    }

    pub fn group_name(&self) -> Option<&GroupName> {
        self.group_name.as_ref()
    }

    pub fn make_load_message(&self) -> RelatedMessage {
        RelatedMessage::LoadRelated {
            group_name: self.group_name.clone(),
            page: self.page,
        }
    }

    pub fn loaded(&mut self, entries: Vec<RelatedArticleEntry>) {
        if self.filter_dirty {
            self.entries = entries;
            self.filter_dirty = false;
        } else {
            self.entries.extend(entries);
        }
    }

    pub fn set_group_name(&mut self, group_name: Option<GroupName>) -> RelatedMessage {
        self.group_name = group_name;
        self.filter_dirty = true;
        self.make_load_message()
    }
}

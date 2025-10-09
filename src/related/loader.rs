use super::{RelatedEntry, crawl::RelatedCrawl};
use crate::{
    crawl::Crawler,
    database::Database,
    name::{ArticleName, GroupName},
    task::Task,
};
use anyhow::Result;
use futures::StreamExt;
use std::{collections::HashMap, str::FromStr};

type TaskInput = (Option<GroupName>, u8);
type TaskContext = (Crawler, Database, ArticleName);
type TaskOutput = Result<Vec<RelatedEntry>>;

pub struct RelatedLoader {
    task: Task<TaskInput, TaskContext, TaskOutput>,
    entries: Vec<RelatedEntry>,
    next_page: Option<u8>,
    filter_group: Option<GroupName>,
    filter_just_changed: bool,
}

impl RelatedLoader {
    pub fn new(crawler: Crawler, database: Database, orig_article_name: ArticleName) -> Self {
        Self {
            task: Task::new(
                (crawler, database, orig_article_name),
                |(group_name, page), (crawler, database, article_name)| async move {
                    let names = crawler
                        .crawl(RelatedCrawl {
                            article_name,
                            group_name,
                            page,
                        })
                        .await?;

                    let mut verdicts = database.get_verdicts();
                    let verdicts = {
                        let mut out = HashMap::new();
                        while let Some(verdict_entry) = verdicts.next().await {
                            let verdict_entry = verdict_entry?;
                            let name = ArticleName::from_str(&verdict_entry.name)?;
                            out.insert(name, verdict_entry.verdict);
                        }
                        out
                    };

                    let out = names
                        .into_iter()
                        .map(|name| {
                            let verdict = verdicts.get(&name).copied();
                            RelatedEntry { name, verdict }
                        })
                        .collect();

                    Ok(out)
                },
            ),
            entries: Vec::new(),
            next_page: Some(1),
            filter_group: None,
            filter_just_changed: false,
        }
    }

    pub fn on_start(&mut self) {
        self.task.run((None, 1));
    }

    pub fn tick(&mut self) -> Result<()> {
        if self.entries.is_empty() && !self.task.is_running() {
            if let Some(page) = self.next_page {
                self.task.run((self.filter_group.clone(), page));
            }
        }

        if let Some(entries) = self.task.output() {
            let entries = entries?;

            if entries.is_empty() {
                self.next_page = None;
            } else {
                if self.filter_just_changed {
                    self.entries = entries;
                    self.filter_just_changed = false;
                } else {
                    self.entries.extend(entries);
                }
                if let Some(page) = self.next_page {
                    self.next_page = Some(page + 1);
                }
            }
        }

        Ok(())
    }

    pub fn entries(&self) -> &[RelatedEntry] {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut [RelatedEntry] {
        &mut self.entries
    }

    pub fn filter_group(&self) -> Option<&GroupName> {
        self.filter_group.as_ref()
    }

    pub fn set_filter_group(&mut self, filter_group: Option<GroupName>) {
        self.filter_group = filter_group.clone();
        self.filter_just_changed = true;
        self.next_page = Some(1);
        self.task.run((filter_group, 1));
    }
}

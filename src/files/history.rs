use super::verdict::Verdict;
use crate::{list::ArticleMap, name::ArticleName};
use anyhow::{Context, Result};
use camino::Utf8Path;
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug)]
pub struct History {
    path: Box<Utf8Path>,
    map: ArticleMap<HistoryEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HistoryEntry {
    pub verdict: Verdict,
}

impl History {
    pub async fn new() -> Result<Self> {
        let dir = super::dir().await?;
        let path = dir.join("history.json").into_boxed_path();
        let map = read_map(&path).await.unwrap_or_default();

        Ok(Self { path, map })
    }

    pub fn has(&self, article: &ArticleName) -> bool {
        self.map.has(article)
    }

    pub fn insert(&mut self, article: ArticleName, verdict: Verdict) -> Option<Verdict> {
        self.map
            .insert(article, HistoryEntry { verdict })
            .map(|h| h.verdict)
    }

    pub async fn flush(&self) -> Result<()> {
        let text = serde_json::to_string(&self.map).context("failed to serialize history")?;
        fs::write(self.path.as_ref(), text)
            .await
            .context("failed to write history")
    }
}

async fn read_map(path: &Utf8Path) -> Option<ArticleMap<HistoryEntry>> {
    let text = fs::read_to_string(path).await.ok()?;
    match serde_json::from_str(&text) {
        Ok(history) => Some(history),
        Err(error) => {
            tracing::error!(%error, "failed to parse history");
            None
        }
    }
}

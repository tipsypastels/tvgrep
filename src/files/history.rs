use super::verdict::Veredict;
use crate::{list::ArticleMap, name::ArticleName};
use anyhow::Result;
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
    pub verdict: Veredict,
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

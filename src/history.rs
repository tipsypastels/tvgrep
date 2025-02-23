use crate::{list::ArticleMap, storage::StorageDir};
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
    pub veredict: Veredict,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Veredict {
    Yes,
    No,
}

impl History {
    pub async fn new(storage: &StorageDir) -> Result<Self> {
        let path = storage.file("history.json");
        let map = read_map(&path).await.unwrap_or_default();

        Ok(Self { path, map })
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

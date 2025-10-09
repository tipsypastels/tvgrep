mod models;

pub use models::*;

use crate::{dirs::Dirs, name::ArticleName};
use anyhow::{Context, Result};
use futures::Stream;
use sqlx::{Executor, SqlitePool, sqlite::SqlitePoolOptions};
use std::pin::Pin;

const FILENAME: &str = if cfg!(debug_assertions) {
    "dev.sqlite"
} else {
    "db.sqlite"
};

#[derive(Clone)]
pub struct Database(SqlitePool);

impl Database {
    pub async fn new(dirs: &Dirs) -> Result<Self> {
        let path = dirs.join(FILENAME).await?;
        let pool = SqlitePoolOptions::new()
            .connect(path.as_str())
            .await
            .context("database connection error")?;

        pool.execute(
            r#"
            CREATE TABLE IF NOT EXISTS article_verdicts (
                name TEXT PRIMARY KEY NOT NULL,
                verdict TEXT CHECK(verdict IN ('y', 'n', 'i')) NOT NULL
            );
        "#,
        )
        .await?;

        Ok(Self(pool))
    }

    pub fn get_verdicts(
        &self,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<ArticleVerdict>> + Send + '_>> {
        sqlx::query_as(r#"SELECT * FROM article_verdicts"#).fetch(&self.0)
    }

    pub async fn set_verdict(&self, name: ArticleName, verdict: Verdict) -> sqlx::Result<()> {
        sqlx::query(r#"INSERT OR REPLACE INTO article_verdicts VALUES (?, ?)"#)
            .bind(name.to_string())
            .bind(verdict)
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub async fn unset_verdict(&self, name: ArticleName) -> sqlx::Result<()> {
        sqlx::query(r#"DELETE FROM article_verdicts WHERE name = ?"#)
            .bind(name.to_string())
            .execute(&self.0)
            .await?;
        Ok(())
    }
}

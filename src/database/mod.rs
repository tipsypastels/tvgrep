use crate::dirs::Dirs;
use anyhow::{Context, Result};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

const FILENAME: &str = if cfg!(debug_assertions) {
    "dev.sqlite"
} else {
    "db.sqlite"
};

pub struct Database(SqlitePool);

impl Database {
    pub async fn new(dirs: &Dirs) -> Result<Self> {
        let path = dirs.join(FILENAME).await?;
        let pool = SqlitePoolOptions::new()
            .connect(path.as_str())
            .await
            .context("database connection error")?;

        Ok(Self(pool))
    }
}

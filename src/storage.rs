use anyhow::{Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use tokio::fs;

#[derive(Debug)]
pub struct StorageDir {
    dir: Box<Utf8Path>,
}

impl StorageDir {
    pub async fn new() -> Result<Self> {
        let dir = data_dir()?.join("tvgrep").into_boxed_path();

        if !fs::try_exists(dir.as_ref()).await? {
            fs::create_dir(dir.as_ref())
                .await
                .context("failed to create storage dir")?;
            tracing::debug!(%dir, "created storage dir");
        }

        tracing::debug!(%dir, "storage dir");
        Ok(Self { dir })
    }

    pub fn file(&self, path: &str) -> Box<Utf8Path> {
        self.dir.join(path).into_boxed_path()
    }
}

fn data_dir() -> Result<Utf8PathBuf> {
    dirs::data_dir()
        .context("no data dir")?
        .try_into()
        .context("data dir not UTF-8")
}

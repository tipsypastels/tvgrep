mod history;
mod verdict;

use anyhow::{Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use tokio::{fs, sync::OnceCell};

pub use history::History;
pub use verdict::Verdict;

static DIR: OnceCell<Box<Utf8Path>> = OnceCell::const_new();

#[cfg(debug_assertions)]
const DIR_NAME: &str = "tvgrep_dev";

#[cfg(not(debug_assertions))]
const DIR_NAME: &str = "tvgrep";

async fn dir() -> Result<&'static Box<Utf8Path>> {
    DIR.get_or_try_init(async || {
        let data_dir: Utf8PathBuf = dirs::data_dir()
            .context("no data dir")?
            .try_into()
            .context("data dir not UTF-8")?;

        let dir = data_dir.join(DIR_NAME).into_boxed_path();

        if !fs::try_exists(dir.as_ref()).await? {
            fs::create_dir(dir.as_ref())
                .await
                .context("failed to create storage dir")?;
        }

        anyhow::Ok(dir)
    })
    .await
}

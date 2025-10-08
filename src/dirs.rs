use anyhow::{Context, Result};
use camino::Utf8Path;

pub struct Dirs {
    #[cfg(not(debug_assertions))]
    base: Box<camino::Utf8Path>,
    _priv: (),
}

impl Dirs {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            #[cfg(not(debug_assertions))]
            base: {
                let data_dir: camino::Utf8PathBuf = dirs::data_dir()
                    .context("no data dir")?
                    .try_into()
                    .context("data dir not UTF-8")?;
                let dir = data_dir.join("tvgrep").into_boxed_path();

                if !tokio::fs::try_exists(dir.as_ref()).await? {
                    tokio::fs::create_dir(dir.as_ref())
                        .await
                        .context("failed to create tvgrep dir")?;
                }

                dir
            },
            _priv: (),
        })
    }

    pub async fn join(&self, path: impl AsRef<Utf8Path>) -> Result<Box<Utf8Path>> {
        #[cfg(debug_assertions)]
        let path = Box::<Utf8Path>::from(path.as_ref());

        #[cfg(not(debug_assertions))]
        let path = self.base.join(path).into_boxed_path();

        if !tokio::fs::try_exists(path.as_ref())
            .await
            .with_context(|| format!("failed to check if {path} exists"))?
        {
            tokio::fs::File::create(path.as_ref())
                .await
                .with_context(|| format!("failed to create {path}"))?;
        }

        Ok(path)
    }
}

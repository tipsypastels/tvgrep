use anyhow::Result;
use dialoguer::Select;
use std::fmt::{self, Display};
use tokio::task::spawn_blocking;

pub fn link<N: Display, U: Display>(name: N, url: U) -> impl Display {
    struct Link<N, U>(N, U);
    impl<N: Display, U: Display> Display for Link<N, U> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1B]8;;{}\x1B\\{}\x1B]8;;\x1B\\", self.1, self.0)
        }
    }
    Link(name, url)
}

pub async fn prompt(choices: &'static [&'static str]) -> Result<&'static str> {
    Ok(choices[spawn_blocking(|| Select::new().items(choices).default(0).interact()).await??])
}

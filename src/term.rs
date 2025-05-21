use anyhow::Result;
use console::{Key, Term};
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

#[derive(Debug, Copy, Clone)]
pub enum VerdictPrompt {
    Yes,
    No,
    Skip,
    Quit,
}

pub async fn verdict_prompt() -> Result<VerdictPrompt> {
    Ok(spawn_blocking(|| {
        let term = Term::stderr();

        term.write_line("[y = yes, n = no, s = skip, q = quit]")?;
        term.hide_cursor()?;
        term.flush()?;

        let choice = loop {
            match term.read_key()? {
                Key::Char('y') | Key::Char('Y') => break VerdictPrompt::Yes,
                Key::Char('n') | Key::Char('N') => break VerdictPrompt::No,
                Key::Char('s') | Key::Char('S') => break VerdictPrompt::Skip,
                Key::Char('q') | Key::Char('Q') => break VerdictPrompt::Quit,
                _ => {}
            }
        };

        term.clear_line()?;
        term.show_cursor()?;
        term.flush()?;

        Ok::<_, std::io::Error>(choice)
    })
    .await??)
}

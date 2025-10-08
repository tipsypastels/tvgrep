use crate::event::Events;
use anyhow::{Context, Result};
use crossterm::event::Event;
use ratatui::{DefaultTerminal, buffer::Buffer, layout::Rect};

pub trait App {
    async fn tick(&mut self) -> Result<()>;
    async fn handle(&mut self, event: Event, quit: &mut bool) -> Result<()>;
    fn render(&mut self, area: Rect, buf: &mut Buffer);

    async fn on_quit(&mut self) -> Result<()> {
        Ok(())
    }

    async fn run(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        let mut events = Events::new();
        let mut quit = false;

        while !quit {
            // self.render(term).await.context("render error")?;
            term.draw(|frame| self.render(frame.area(), frame.buffer_mut()))
                .context("render error")?;

            match events.next().await? {
                crate::event::Event::Tick => {
                    self.tick().await.context("tick error")?;
                }
                crate::event::Event::Term(event) => {
                    self.handle(event, &mut quit).await.context("event error")?;
                }
            }
        }

        self.on_quit().await.context("on_quit error")?;
        Ok(())
    }
}

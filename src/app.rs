use anyhow::{Context, Result};
use crossterm::event::{Event as TermEvent, EventStream};
use futures::{FutureExt, StreamExt};
use ratatui::{DefaultTerminal, buffer::Buffer, layout::Rect};
use std::time::Duration;
use tokio::sync::mpsc;

pub trait App {
    async fn tick(&mut self) -> Result<()>;
    async fn handle(&mut self, event: TermEvent, quit: &mut bool) -> Result<()>;
    fn render(&mut self, area: Rect, buf: &mut Buffer);

    async fn on_quit(&mut self) -> Result<()> {
        Ok(())
    }

    async fn run(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        let mut events = Events::new();
        let mut quit = false;

        while !quit {
            term.draw(|frame| self.render(frame.area(), frame.buffer_mut()))
                .context("render error")?;

            match events.next().await? {
                Event::Tick => {
                    self.tick().await.context("tick error")?;
                }
                Event::Term(event) => {
                    self.handle(event, &mut quit).await.context("event error")?;
                }
            }
        }

        self.on_quit().await.context("on_quit error")?;
        Ok(())
    }
}

pub const FPS: f64 = 30.0;

enum Event {
    Tick,
    Term(TermEvent),
}

struct Events {
    rx: mpsc::UnboundedReceiver<Event>,
}

impl Events {
    fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let actor = Actor { tx };

        tokio::spawn(actor.run());
        Self { rx }
    }

    async fn next(&mut self) -> Result<Event> {
        self.rx.recv().await.context("failed to receive event")
    }
}

struct Actor {
    tx: mpsc::UnboundedSender<Event>,
}

impl Actor {
    async fn run(self) -> Result<()> {
        let tick_rate = Duration::from_secs_f64(1.0 / FPS);
        let mut reader = EventStream::new();
        let mut tick = tokio::time::interval(tick_rate);

        loop {
            tokio::select! {
                _ = self.tx.closed() => {
                    return Ok(())
                }
                _ = tick.tick() => {
                    _ = self.tx.send(Event::Tick);
                }
                Some(Ok(event)) = reader.next().fuse() => {
                    _ = self.tx.send(Event::Term(event));
                }
            }
        }
    }
}

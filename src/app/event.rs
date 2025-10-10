use anyhow::{Context, Result};
use crossterm::event::{Event as TermEvent, EventStream};
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;

pub const FPS: f64 = 30.0;

pub enum Event {
    Tick,
    Term(TermEvent),
}

pub struct Events {
    rx: mpsc::UnboundedReceiver<Event>,
}

impl Events {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let actor = Actor { tx };

        tokio::spawn(actor.run());

        Self { rx }
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.rx.recv().await.context("failed to receive event")
    }
}

struct Actor {
    tx: mpsc::UnboundedSender<Event>,
}

impl Actor {
    async fn run(self) {
        let tick_rate = Duration::from_secs_f64(1.0 / FPS);
        let mut reader = EventStream::new();
        let mut tick = tokio::time::interval(tick_rate);

        loop {
            tokio::select! {
                _ = self.tx.closed() => {
                    return;
                }
                _ = tick.tick() => {
                    let _ = self.tx.send(Event::Tick);
                }
                Some(Ok(event)) = reader.next().fuse() => {
                    let _ = self.tx.send(Event::Term(event));
                }
            }
        }
    }
}

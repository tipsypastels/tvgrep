use anyhow::{Context, Result};
use crossterm::event::{Event as TermEvent, EventStream};
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;

pub const FPS: f64 = 30.0;

pub enum Event<M: MessengerNamespace> {
    Tick,
    Term(TermEvent),
    MessageOutput(M::Output),
}

pub struct Events<M: MessengerNamespace> {
    message_tx: mpsc::UnboundedSender<M::Input>,
    event_rx: mpsc::UnboundedReceiver<Event<M>>,
}

impl<M: MessengerNamespace> Events<M> {
    pub fn new<F>(message_context: M::Context, on_message: fn(M::Input, M::Context) -> F) -> Self
    where
        F: Future<Output = M::Output> + Send + 'static,
    {
        let (message_tx, message_rx) = mpsc::unbounded_channel();
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        let actor = Actor {
            message_rx,
            event_tx,
        };

        tokio::spawn(actor.run(message_context, on_message));

        Self {
            message_tx,
            event_rx,
        }
    }

    pub fn send_message(&self, message: M::Input) {
        let _ = self.message_tx.send(message);
    }

    pub async fn next(&mut self) -> Result<Event<M>> {
        self.event_rx
            .recv()
            .await
            .context("failed to receive event")
    }
}

struct Actor<M: MessengerNamespace> {
    message_rx: mpsc::UnboundedReceiver<M::Input>,
    event_tx: mpsc::UnboundedSender<Event<M>>,
}

impl<M: MessengerNamespace> Actor<M> {
    async fn run<F>(
        mut self,
        message_context: M::Context,
        on_message: fn(M::Input, M::Context) -> F,
    ) -> Result<()>
    where
        F: Future<Output = M::Output> + Send + 'static,
    {
        let tick_rate = Duration::from_secs_f64(1.0 / FPS);
        let mut reader = EventStream::new();
        let mut tick = tokio::time::interval(tick_rate);

        loop {
            tokio::select! {
                _ = self.event_tx.closed() => {
                    return Ok(())
                }
                _ = tick.tick() => {
                    let _ = self.event_tx.send(Event::Tick);
                }
                Some(message) = self.message_rx.recv() => {
                    let output = on_message(message, message_context.clone()).await;
                    let _ = self.event_tx.send(Event::MessageOutput(output));
                }
                Some(Ok(event)) = reader.next().fuse() => {
                    let _ = self.event_tx.send(Event::Term(event));
                }
            }
        }
    }
}

pub trait MessengerNamespace: Send + 'static {
    type Input: Send + 'static;
    type Output: Send + 'static;
    type Context: Clone + Send + 'static;
}

impl<I, O, C> MessengerNamespace for (I, O, C)
where
    I: Send + 'static,
    O: Send + 'static,
    C: Clone + Send + 'static,
{
    type Input = I;
    type Output = O;
    type Context = C;
}

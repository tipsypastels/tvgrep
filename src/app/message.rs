use anyhow::{Context, Result};
use tokio::sync::mpsc;

pub trait Messenger: Send + 'static {
    type Input: Send + 'static;
    type Output: Send + 'static;
    type Context: Clone + Send + 'static;
}

impl<I, O, C> Messenger for fn(I, C) -> O
where
    I: Send + 'static,
    O: Send + 'static,
    C: Clone + Send + 'static,
{
    type Input = I;
    type Output = O;
    type Context = C;
}

pub struct Messages<M: Messenger> {
    input_tx: mpsc::UnboundedSender<M::Input>,
    output_rx: mpsc::UnboundedReceiver<M::Output>,
    outgoing_cnt: usize,
}

impl<M: Messenger> Messages<M> {
    pub fn new<F>(context: M::Context, handle: fn(M::Input, M::Context) -> F) -> Self
    where
        F: Future<Output = M::Output> + Send + 'static,
    {
        let (input_tx, input_rx) = mpsc::unbounded_channel();
        let (output_tx, output_rx) = mpsc::unbounded_channel();

        let actor = Actor::<M> {
            input_rx,
            output_tx,
        };

        tokio::spawn(actor.run(context, handle));

        Self {
            input_tx,
            output_rx,
            outgoing_cnt: 0,
        }
    }

    pub fn is_loading(&self) -> bool {
        self.outgoing_cnt > 0
    }

    pub fn send(&mut self, message: M::Input) {
        self.input_tx.send(message).ok();
        self.outgoing_cnt += 1;
    }

    pub async fn next(&mut self) -> Result<M::Output> {
        self.output_rx
            .recv()
            .await
            .context("failed to receive message")
            .inspect(|_| {
                self.outgoing_cnt -= 1;
            })
    }

    pub async fn close(&mut self) -> Result<()> {
        while self.outgoing_cnt > 0 {
            self.next().await?;
        }
        Ok(())
    }
}

struct Actor<M: Messenger> {
    input_rx: mpsc::UnboundedReceiver<M::Input>,
    output_tx: mpsc::UnboundedSender<M::Output>,
}

impl<M: Messenger> Actor<M> {
    async fn run<F>(mut self, context: M::Context, handle: fn(M::Input, M::Context) -> F)
    where
        F: Future<Output = M::Output> + Send + 'static,
    {
        while let Some(input) = self.input_rx.recv().await {
            let output = handle(input, context.clone()).await;
            let _ = self.output_tx.send(output);
        }
    }
}

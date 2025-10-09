use std::marker::PhantomData;
use tokio::sync::{mpsc, oneshot};

pub struct Task<I, Ctx, O> {
    tx: mpsc::UnboundedSender<(I, oneshot::Sender<O>)>,
    current_ack: Option<oneshot::Receiver<O>>,
    _ctx: PhantomData<Ctx>,
}

impl<I, Ctx, O> Task<I, Ctx, O>
where
    I: Send + 'static,
    Ctx: Clone + Send + 'static,
    O: Send + 'static,
{
    pub fn new<F, Fut>(ctx: Ctx, f: F) -> Self
    where
        F: Fn(I, Ctx) -> Fut + Send + 'static,
        Fut: Future<Output = O> + Send,
    {
        let (tx, mut rx) = mpsc::unbounded_channel::<(I, oneshot::Sender<O>)>();

        tokio::spawn(async move {
            let ctx = ctx;

            while let Some((input, syn)) = rx.recv().await {
                // TODO: Figure out why passing references is disallowed.
                let output = f(input, ctx.clone()).await;
                let _ = syn.send(output);
            }
        });

        Self {
            tx,
            current_ack: None,
            _ctx: PhantomData,
        }
    }

    pub fn run(&mut self, input: I) {
        let (syn, ack) = oneshot::channel();
        let _ = self.tx.send((input, syn));
        self.current_ack = Some(ack);
    }

    pub fn is_running(&self) -> bool {
        self.current_ack.is_some()
    }

    pub fn output(&mut self) -> Option<O> {
        self.current_ack
            .as_mut()
            .and_then(|ack| ack.try_recv().ok())
    }

    pub async fn close(&mut self) {
        if let Some(ack) = self.current_ack.take() {
            let _ = ack.await;
        }
    }
}

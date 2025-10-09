use std::marker::PhantomData;
use tokio::sync::mpsc;

pub struct TaskPool<Ctx, T> {
    tx: mpsc::UnboundedSender<T>,
    _ctx: PhantomData<Ctx>,
}

impl<Ctx, T> TaskPool<Ctx, T> {
    pub fn new<F, Fut>(ctx: Ctx, f: F) -> Self
    where
        T: Send + 'static,
        Ctx: Clone + Send + 'static,
        F: Fn(Ctx, T) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send,
    {
        let (tx, mut rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            let ctx = ctx;
            while let Some(message) = rx.recv().await {
                f(ctx.clone(), message).await;
            }
        });

        Self {
            tx,
            _ctx: PhantomData,
        }
    }

    pub fn send(&self, message: T) {
        let _ = self.tx.send(message);
    }
}

use std::marker::PhantomData;
use tokio::sync::{mpsc, oneshot};

pub struct Loader<T, Ctx> {
    tx: mpsc::UnboundedSender<oneshot::Sender<T>>,
    current_ack: Option<oneshot::Receiver<T>>,
    _ctx: PhantomData<fn() -> Ctx>,
}

impl<T, Ctx> Loader<T, Ctx> {
    pub fn new<F>(ctx: Ctx, f: fn(&mut Ctx) -> F) -> Self
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
        Ctx: Send + 'static,
    {
        let (tx, mut rx) = mpsc::unbounded_channel::<oneshot::Sender<T>>();

        tokio::spawn(async move {
            let mut ctx = ctx;

            while let Some(syn) = rx.recv().await {
                let result = f(&mut ctx).await;
                let _ = syn.send(result);
            }
        });

        Self {
            tx,
            current_ack: None,
            _ctx: PhantomData,
        }
    }

    pub fn start_loading(&mut self) {
        let (syn, ack) = oneshot::channel();
        let _ = self.tx.send(syn);
        self.current_ack = Some(ack);
    }

    pub fn cancel_loading(&mut self) {
        self.current_ack = None;
    }

    pub fn is_loading(&self) -> bool {
        self.current_ack.is_some()
    }

    pub fn read(&mut self) -> Option<T> {
        self.current_ack
            .as_mut()
            .and_then(|ack| ack.try_recv().ok())
    }
}

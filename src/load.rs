use tokio::sync::{mpsc, oneshot};

pub trait Load: Send + 'static {
    type Output: Send;
    fn load(&mut self) -> impl Future<Output = Self::Output> + Send;
}

pub struct Loader<L: Load> {
    tx: mpsc::UnboundedSender<oneshot::Sender<L::Output>>,
    current_ack: Option<oneshot::Receiver<L::Output>>,
}

impl<L: Load> Loader<L> {
    pub fn new(load: L) -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel::<oneshot::Sender<L::Output>>();

        tokio::spawn(async move {
            let mut load = load;

            while let Some(syn) = rx.recv().await {
                let output = load.load().await;
                let _ = syn.send(output);
            }
        });

        Self {
            tx,
            current_ack: None,
        }
    }

    pub fn start_loading(&mut self) {
        let (syn, ack) = oneshot::channel();
        let _ = self.tx.send(syn);
        self.current_ack = Some(ack);
    }

    pub fn is_loading(&self) -> bool {
        self.current_ack.is_some()
    }

    pub fn read(&mut self) -> Option<L::Output> {
        self.current_ack
            .as_mut()
            .and_then(|ack| ack.try_recv().ok())
    }
}

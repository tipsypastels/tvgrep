use anyhow::Result;
use futures::future::maybe_done;
use tokio::sync::Mutex;
use std::{fmt, sync::Arc};

pub async fn start<'a, T, U, I, RF, DF>(iter: I, run_func: RF, download_func: DF) -> Result<()>
where
    T: fmt::Debug + 'a,
    I: Iterator<Item = &'a T>,
    RF: AsyncFnMut(&'a T, U) -> Result<()>,
    DF: AsyncFn(&'a T) -> Result<U>,
{
    let run_func_lock = Arc::new(Mutex::new(run_func));
    let mut iter = iter
        .map(|item| (item, Box::pin(maybe_done(download_func(item)))))
        .peekable();

    while let Some((item, mut future)) = iter.next() {
        let run_func_lock = run_func_lock.clone();
        let run_current = async move {
            tracing::debug!("late-downloading {item:?}");
            let () = future.as_mut().await;
            let data = future.as_mut().take_output().unwrap()?;

            let mut run_func = run_func_lock.lock().await;
            run_func(item, data).await
        };

        let preload_next_item = iter.peek_mut();
        let preload_next = async move {
            if let Some((item, fut)) = preload_next_item {
                tracing::debug!("preloading {item:?}");
                fut.await;
                tracing::debug!("preloaded {item:?}");
            }
            anyhow::Ok(())
        };

        tokio::try_join!(run_current, preload_next)?;
    }

    Ok(())
}

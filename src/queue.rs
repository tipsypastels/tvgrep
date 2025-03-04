use anyhow::Result;
use futures::future::maybe_done;
use std::{fmt, ops::ControlFlow, sync::Arc};

pub async fn make<'a, T, U, I, Rf, Df>(iter: I, download_func: Df, run_func: Rf) -> Result<()>
where
    T: fmt::Debug + 'a,
    I: Iterator<Item = &'a T>,
    Df: AsyncFn(&'a T) -> Result<U>,
    Rf: AsyncFn(&'a T, U) -> Result<ControlFlow<()>>,
{
    let run_func = Arc::new(run_func);
    let mut iter = iter
        .map(|item| (item, Box::pin(maybe_done(download_func(item)))))
        .peekable();

    while let Some((item, mut future)) = iter.next() {
        let run_func = run_func.clone();
        let run_current = async move {
            let () = future.as_mut().await;
            let data = future.as_mut().take_output().unwrap()?;

            run_func(item, data).await
        };

        let preload_next_item = iter.peek_mut();
        let preload_next = async move {
            if let Some((_, fut)) = preload_next_item {
                fut.await;
            }
            anyhow::Ok(())
        };

        let (cf, _) = tokio::try_join!(run_current, preload_next)?;

        // not using is_break to help type inference
        if matches!(cf, ControlFlow::<(), ()>::Break(_)) {
            break;
        }
    }

    Ok(())
}

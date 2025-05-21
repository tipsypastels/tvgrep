use crate::progress::Progress;
use anyhow::Result;
use futures::future::maybe_done;
use std::{ops::ControlFlow, sync::Arc};

pub async fn make<'a, T, U, I, Rf, Df>(iter: I, download_func: Df, run_func: Rf) -> Result<()>
where
    T: 'a,
    I: Iterator<Item = (usize, &'a T)>,
    Df: AsyncFn(&'a T) -> Result<U>,
    Rf: AsyncFn(&'a T, U, Progress) -> Result<ControlFlow<()>>,
{
    let run_func = Arc::new(run_func);

    let max = iter.size_hint().1;
    let mut iter = iter
        .map(|(i, item)| (i, item, Box::pin(maybe_done(download_func(item)))))
        .peekable();

    while let Some((i, item, mut future)) = iter.next() {
        let run_func = run_func.clone();
        let run_current = async move {
            let () = future.as_mut().await;
            let data = future.as_mut().take_output().unwrap()?;
            let progress = Progress::new(i, max);

            run_func(item, data, progress).await
        };

        let preload_next_item = iter.peek_mut();
        let preload_next = async move {
            if let Some((_, _, fut)) = preload_next_item {
                fut.await;
            }
            anyhow::Ok(())
        };

        let (cf, ()) = tokio::try_join!(run_current, preload_next)?;

        // not using is_break to help type inference
        if matches!(cf, ControlFlow::<(), ()>::Break(_)) {
            break;
        }
    }

    Ok(())
}

use poll_promise::Promise;
use std::future::Future;

pub fn spawn<F>(ctx: &egui::Context, runtime: &tokio::runtime::Handle, future: F) -> Promise<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let ctx     = ctx.clone();
    let _guard  = runtime.enter();
    Promise::spawn_async(async move {
        let result = future.await;
        ctx.request_repaint();
        result
    })
}

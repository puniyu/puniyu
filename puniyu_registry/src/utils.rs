use std::pin::Pin;
type TaskFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
pub(crate) async fn execute_tasks(
    init_futures: Vec<(String, TaskFuture)>,
) -> Vec<Result<(), tokio::task::JoinError>> {
    let tasks: Vec<_> = init_futures
        .into_iter()
        .map(|(_plugin_name, future)| {
            tokio::spawn(async move {
                future.await;
            })
        })
        .collect();

    futures::future::join_all(tasks).await
}

use async_trait::async_trait;

use crate::handler::HandlerResult;

/// 定时任务构建器
///
/// # 示例
/// ```
/// use async_trait::async_trait;
/// use puniyu_types::task::Task;
///
/// struct MyTask;
///
/// #[async_trait]
/// impl Task for MyTask {
///     fn name(&self) -> &'static str {
///         "my_task"
///     }
///     
///     fn cron(&self) -> &'static str {
///         "0 0 * * * *" // 每小时执行一次
///     }
///     
///     async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///         println!("Task running!");
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait Task: Send + Sync + 'static {
	/// 任务名称
	fn name(&self) -> &'static str;

	/// cron 表达式
	fn cron(&self) -> &'static str;

	/// 执行任务
	async fn run(&self) -> HandlerResult;
}

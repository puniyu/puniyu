use std::pin::Pin;

/// 定时任务
pub trait TaskBuilder: Send + Sync + 'static {
	/// 任务名称
	fn name(&self) -> &'static str;

	/// cron 表达式
	fn cron(&self) -> &'static str;

	/// 执行任务
	fn run(&self) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
}

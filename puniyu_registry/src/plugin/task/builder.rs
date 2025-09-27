use async_trait::async_trait;
/// 定时任务
#[async_trait]
pub trait TaskBuilder: Send + Sync + 'static {
	/// 任务名称
	fn name(&self) -> &'static str;

	/// cron 表达式
	fn cron(&self) -> &'static str;

	/// 执行任务
	async fn run(&self);
}

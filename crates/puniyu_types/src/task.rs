use async_trait::async_trait;

use crate::handler::HandlerResult;

#[derive(Debug, Clone)]
pub enum TaskId {
	Index(u64),
	Name(String),
}

impl From<u64> for TaskId {
	fn from(value: u64) -> Self {
		Self::Index(value)
	}
}
impl From<String> for TaskId {
	fn from(value: String) -> Self {
		Self::Name(value)
	}
}

impl From<&str> for TaskId {
	fn from(value: &str) -> Self {
		Self::Name(value.to_string())
	}
}

pub struct Task {
	/// 任务名称
	pub name: &'static str,
	/// 任务cron表达式
	pub cron: &'static str,
}

#[async_trait]
/// 定时任务构建器
///
/// # 示例
/// ```
/// use async_trait::async_trait;
/// use puniyu_types::task::TaskBuilder;
///
/// struct MyTask;
///
/// #[async_trait]
/// impl TaskBuilder for MyTask {
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
pub trait TaskBuilder: Send + Sync + 'static {
	/// 任务名称
	fn name(&self) -> &'static str;

	/// cron 表达式
	fn cron(&self) -> &'static str;

	/// 执行任务
	async fn run(&self) -> HandlerResult;
}

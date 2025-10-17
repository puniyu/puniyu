use async_trait::async_trait;

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

#[derive(Debug, Clone)]
pub struct Task {
	pub name: &'static str,
	pub cron: &'static str,
}

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

impl From<Box<dyn TaskBuilder>> for Task {
	fn from(task_builder: Box<dyn TaskBuilder>) -> Self {
		Self { name: task_builder.name(), cron: task_builder.cron() }
	}
}

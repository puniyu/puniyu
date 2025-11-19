mod builder;
mod registry;

pub use builder::TaskBuilder;
pub use registry::TaskRegistry;

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
	pub cron: &'static str
}


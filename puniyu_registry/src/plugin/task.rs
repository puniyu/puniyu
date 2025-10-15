use std::sync::Arc;
use tokio::sync::OnceCell;
use tokio_cron_scheduler::JobScheduler;
pub mod builder;
pub mod manger;
pub mod registry;

pub(crate) static SCHEDULER: OnceCell<Arc<JobScheduler>> = OnceCell::const_new();

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

pub async fn init_scheduler() {
	SCHEDULER
		.get_or_init(|| async {
			let sched = JobScheduler::new().await.unwrap();
			Arc::new(sched)
		})
		.await;
}

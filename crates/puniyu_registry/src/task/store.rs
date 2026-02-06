use super::types::TaskInfo;
use crate::{Error, Result};
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};
use puniyu_logger::warn;
use tokio::sync::OnceCell;
use tokio_cron_scheduler::JobScheduler;
use uuid::Uuid;

static TASK_ID: AtomicU64 = AtomicU64::new(0);
static SCHEDULER: OnceCell<Arc<JobScheduler>> = OnceCell::const_new();

#[derive(Default)]
pub(crate) struct TaskStore {
	tasks: Arc<RwLock<HashMap<u64, TaskInfo>>>,
	uuid_map: Arc<RwLock<HashMap<u64, Uuid>>>,
}

impl TaskStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub async fn get_scheduler(&self) -> &'static Arc<JobScheduler> {
		SCHEDULER
			.get_or_init(|| async {
				let sched = JobScheduler::new().await.unwrap();
				Arc::new(sched)
			})
			.await
	}

	pub async fn insert(&self, task: TaskInfo) -> Result<u64> {
		let index = TASK_ID.fetch_add(1, Ordering::Relaxed);
		let mut map = self.tasks.write().expect("Failed to acquire lock");

		if map
			.values()
			.any(|v| v.plugin_id == task.plugin_id && v.builder.name() == task.builder.name())
		{
			return Err(Error::Exists("Task".to_string()));
		}

		let job = tokio_cron_scheduler::Job::from(task.clone());
		let scheduler = self.get_scheduler().await;
		let uuid = scheduler.add(job).await.map_err(|e| Error::NotFound(e.to_string()))?;

		self.uuid_map.write().expect("Failed to acquire lock").insert(index, uuid);
		map.insert(index, task);

		Ok(index)
	}

	pub async fn remove(&self, index: u64) -> Result<()> {
		let mut tasks = self.tasks.write().expect("Failed to acquire lock");
		let mut uuid_map = self.uuid_map.write().expect("Failed to acquire lock");

		if tasks.get(&index).is_none() {
			return Err(Error::NotFound("Task".to_string()));
		}

		if let Some(uuid) = uuid_map.remove(&index) {
			let scheduler = self.get_scheduler().await;
			if scheduler.remove(&uuid).await.is_err() {
				return Err(Error::NotFound("Task".to_string()));
			}
		}

		tasks.remove(&index);
		Ok(())
	}

	pub fn all(&self) -> Vec<TaskInfo> {
		let map = self.tasks.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	pub fn raw(&self) -> Arc<RwLock<HashMap<u64, TaskInfo>>> {
		self.tasks.clone()
	}
}

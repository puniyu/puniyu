use crate::error::Error;
use crate::TaskInfo;
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};
use uuid::Uuid;

static TASK_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
pub(crate) struct TaskStore {
	tasks: Arc<RwLock<HashMap<u64, TaskInfo>>>,
	uuid_map: Arc<RwLock<HashMap<u64, Uuid>>>,
}

impl TaskStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub async fn insert(&self, info: TaskInfo) -> Result<u64, Error> {
		let index = TASK_ID.fetch_add(1, Ordering::Relaxed);
		{
			let map = self.tasks.read().expect("Failed to acquire read lock");
			if map
				.values()
				.any(|v| v.plugin_id == info.plugin_id && v.task.name() == info.task.name())
			{
				return Err(Error::Exists("Task".to_string()));
			}
		}

		let job = tokio_cron_scheduler::Job::from(&info);
		let scheduler = crate::get_scheduler()?;
		let uuid = scheduler
			.lock()
			.await
			.add(job)
			.await
			.map_err(|e| Error::Start(e.to_string()))?;

		{
			let mut map = self.tasks.write().expect("Failed to acquire write lock");
			let mut uuid_map = self.uuid_map.write().expect("Failed to acquire write lock");

			uuid_map.insert(index, uuid);
			map.insert(index, info);
		}

		Ok(index)
	}

	pub async fn remove(&self, index: u64) -> Result<(), Error> {
		let uuid = {
			let mut uuid_map = self.uuid_map.write().expect("Failed to acquire lock");
			uuid_map.remove(&index)
		};

		if uuid.is_none() {
			return Err(Error::NotFound("Task".to_string()));
		}

		#[allow(clippy::unwrap_used)]
		let uuid = uuid.unwrap();

		let scheduler = crate::get_scheduler()?;
		if scheduler.lock().await.remove(&uuid).await.is_err() {
			return Err(Error::NotFound("Task".to_string()));
		}

		let mut tasks = self.tasks.write().expect("Failed to acquire lock");
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

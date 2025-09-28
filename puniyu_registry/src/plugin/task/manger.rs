use crate::plugin::task::{TaskId, get_scheduler, registry::TaskRegistry};
use std::{
	collections::HashMap,
	sync::{
		Arc, LazyLock, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};
use uuid::Uuid;

static TASK_STORE: LazyLock<Arc<RwLock<TaskStore>>> =
	LazyLock::new(|| Arc::new(RwLock::new(TaskStore::default())));

static TASK_MANAGER: LazyLock<Arc<RwLock<HashMap<u64, Uuid>>>> =
	LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

static TASK_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
struct TaskStore(HashMap<u64, Arc<TaskRegistry>>);

impl TaskStore {
	pub fn insert_task(task: TaskRegistry) -> u64 {
		let id = TASK_INDEX.fetch_add(1, Ordering::Relaxed);
		TASK_STORE.write().unwrap().0.insert(id, Arc::new(task));
		id
	}

	pub fn get_task<T: Into<TaskId>>(task: T) -> Option<Arc<TaskRegistry>> {
		let store = TASK_STORE.read().unwrap();
		match task.into() {
			TaskId::Index(index) => store.0.get(&index).cloned(),
			TaskId::Name(name) => {
				store.0.values().find(|r| r.builder.name() == name.as_str()).cloned()
			}
		}
	}

	pub fn get_tasks() -> Vec<(u64, Arc<TaskRegistry>)> {
		TASK_STORE.read().unwrap().0.iter().map(|(id, reg)| (*id, reg.clone())).collect()
	}

	pub fn remove_tasks_by_plugin(plugin: &str) -> Vec<u64> {
		let mut store = TASK_STORE.write().unwrap();
		let ids: Vec<u64> =
			store.0.iter().filter(|(_, r)| r.plugin_name == plugin).map(|(id, _)| *id).collect();
		for id in &ids {
			store.0.remove(id);
		}
		ids
	}

	pub fn remove_task_by_id(id: u64) -> Option<Arc<TaskRegistry>> {
		TASK_STORE.write().unwrap().0.remove(&id)
	}
}

pub struct TaskManager;

impl TaskManager {
	pub async fn add_task(task: TaskRegistry) -> u64 {
		let id = TaskStore::insert_task(task);
		let registry = TaskStore::get_task(id).unwrap();

		let job = tokio_cron_scheduler::Job::from(TaskRegistry {
			plugin_name: registry.plugin_name,
			builder: registry.builder.clone(),
		});

		let scheduler = get_scheduler().await;
		let uuid = scheduler.add(job).await.unwrap();
		TASK_MANAGER.write().unwrap().insert(id, uuid);

		id
	}

	pub async fn remove_task<T>(task: T) -> bool
	where
		T: Into<TaskId>,
	{
		match task.into() {
			TaskId::Index(id) => {
				let uuid = { TASK_MANAGER.write().unwrap().remove(&id) };
				let _ = TaskStore::remove_task_by_id(id);

				if let Some(uuid) = uuid {
					let scheduler = get_scheduler().await;
					let _ = scheduler.remove(&uuid).await;
					true
				} else {
					false
				}
			}
			TaskId::Name(plugin_name) => {
				let task_ids = TaskStore::remove_tasks_by_plugin(&plugin_name);
				if task_ids.is_empty() {
					return false;
				}

				let mut removed = 0;
				for id in task_ids {
					let uuid = { TASK_MANAGER.write().unwrap().remove(&id) };
					if let Some(uuid) = uuid {
						let scheduler = get_scheduler().await;
						let _ = scheduler.remove(&uuid).await;
						removed += 1;
					}
				}
				removed > 0
			}
		}
	}
}

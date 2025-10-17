use crate::Task;
use puniyu_task_builder::TaskId;
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

pub(crate) static TASK_MANAGER: LazyLock<Arc<RwLock<HashMap<u64, Uuid>>>> =
	LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

static TASK_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
pub(crate) struct TaskStore(HashMap<u64, Arc<Task>>);

impl TaskStore {
	pub fn insert_task(task: Task) -> u64 {
		let id = TASK_INDEX.fetch_add(1, Ordering::Relaxed);
		TASK_STORE.write().unwrap().0.insert(id, Arc::new(task));
		id
	}

	pub fn get_task<T: Into<TaskId>>(task: T) -> Option<Arc<Task>> {
		let store = TASK_STORE.read().unwrap();
		match task.into() {
			TaskId::Index(index) => store.0.get(&index).cloned(),
			TaskId::Name(name) => {
				store.0.values().find(|r| r.builder.name() == name.as_str()).cloned()
			}
		}
	}

	pub fn get_tasks() -> Vec<(u64, Arc<Task>)> {
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

	pub fn remove_task_by_id(id: u64) -> Option<Arc<Task>> {
		TASK_STORE.write().unwrap().0.remove(&id)
	}
}

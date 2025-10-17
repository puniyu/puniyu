use crate::{SCHEDULER, Task, store};
use puniyu_task_builder::TaskId;
use std::sync::Arc;

pub struct TaskRegistry;

impl TaskRegistry {
	pub async fn add_task(task: Task) -> u64 {
		let id = store::TaskStore::insert_task(task);
		let registry = store::TaskStore::get_task(id).unwrap();

		let job = tokio_cron_scheduler::Job::from(Task {
			plugin_name: registry.plugin_name,
			builder: registry.builder.clone(),
		});

		let scheduler = SCHEDULER.get().unwrap();
		let uuid = scheduler.add(job).await.unwrap();
		store::TASK_MANAGER.write().unwrap().insert(id, uuid);
		id
	}

	pub async fn remove_task(task: impl Into<TaskId>) -> bool {
		match task.into() {
			TaskId::Index(id) => {
				let uuid = { store::TASK_MANAGER.write().unwrap().remove(&id) };
				let _ = store::TaskStore::remove_task_by_id(id);

				if let Some(uuid) = uuid {
					let scheduler = SCHEDULER.get().unwrap();
					scheduler.remove(&uuid).await.is_ok()
				} else {
					false
				}
			}
			TaskId::Name(plugin_name) => {
				let task_ids = store::TaskStore::remove_tasks_by_plugin(&plugin_name);
				if task_ids.is_empty() {
					return false;
				}

				let mut removed = 0;
				for id in task_ids {
					let uuid = { store::TASK_MANAGER.write().unwrap().remove(&id) };
					if let Some(uuid) = uuid {
						let scheduler = SCHEDULER.get().unwrap();
						let _ = scheduler.remove(&uuid).await;
						removed += 1;
					}
				}
				removed > 0
			}
		}
	}

	pub fn get_all_tasks() -> Vec<(u64, Arc<Task>)> {
		store::TaskStore::get_tasks()
	}
}

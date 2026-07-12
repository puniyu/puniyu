mod store;

use crate::error::Error;
use crate::registry::store::TaskStore;
use crate::{SCHEDULER, TaskId, TaskInfo};
use std::sync::Arc;
use std::sync::LazyLock;

static STORE: LazyLock<TaskStore> = LazyLock::new(TaskStore::new);

/// 任务注册表
pub struct TaskRegistry;

impl<'t> TaskRegistry {
	/// 注册任务
	pub async fn register(plugin_id: u64, task: impl Into<Arc<dyn crate::Task>>) -> Result<u64, Error> {
		if SCHEDULER.get().is_none() {
			return Err(Error::NotInitialized);
		}
		let info = TaskInfo { plugin_id, task: task.into() };
		STORE.insert(info).await
	}

	/// 卸载任务
	pub async fn unregister<T>(task: T) -> Result<(), Error>
	where
		T: Into<TaskId<'t>>,
	{
		if SCHEDULER.get().is_none() {
			return Err(Error::NotInitialized);
		}
		let task = task.into();
		match task {
			TaskId::Index(id) => Self::unregister_with_index(id).await,
			TaskId::Name(name) => Self::unregister_with_task_name(name.as_ref()).await,
		}
	}

	/// 通过索引卸载任务
	pub async fn unregister_with_index(index: u64) -> Result<(), Error> {
		if SCHEDULER.get().is_none() {
			return Err(Error::NotInitialized);
		}
		STORE.remove(index).await
	}

	/// 通过任务名称卸载任务
	pub async fn unregister_with_task_name(name: &str) -> Result<(), Error> {
		if SCHEDULER.get().is_none() {
			return Err(Error::NotInitialized);
		}
		let task_ids: Vec<u64> = {
			let raw = STORE.raw();
			let map = raw.read().expect("Failed to acquire lock");
			map.iter()
				.filter(|(_, info)| info.task.name() == name)
				.map(|(id, _)| *id)
				.collect()
		};

		if task_ids.is_empty() {
			return Err(Error::NotFound("Task".to_string()));
		}

		for id in task_ids {
			STORE.remove(id).await?;
		}
		Ok(())
	}

	/// 通过插件 ID 卸载所有关联任务
	pub async fn unregister_with_plugin_id(plugin_id: u64) -> Result<(), Error> {
		if SCHEDULER.get().is_none() {
			return Err(Error::NotInitialized);
		}
		let task_ids: Vec<u64> = {
			let raw = STORE.raw();
			let map = raw.read().expect("Failed to acquire lock");
			map.iter().filter(|(_, info)| info.plugin_id == plugin_id).map(|(id, _)| *id).collect()
		};

		if task_ids.is_empty() {
			return Err(Error::NotFound("Task".to_string()));
		}

		for id in task_ids {
			STORE.remove(id).await?;
		}
		Ok(())
	}

	/// 查询任务
	pub fn get<T>(task: T) -> Vec<TaskInfo>
	where
		T: Into<TaskId<'t>>,
	{
		let task = task.into();
		match task {
			TaskId::Index(id) => Self::get_with_index(id).into_iter().collect(),
			TaskId::Name(name) => Self::get_with_task_name(name.as_ref()),
		}
	}

	/// 通过索引查询任务
	pub fn get_with_index(index: u64) -> Option<TaskInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	/// 通过任务名称查询任务
	pub fn get_with_task_name(name: &str) -> Vec<TaskInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.task.name() == name).cloned().collect()
	}

	/// 通过插件 ID 查询任务
	pub fn get_with_plugin_id(plugin_id: u64) -> Vec<TaskInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.plugin_id == plugin_id).cloned().collect()
	}

	/// 获取所有任务
	pub fn all() -> Vec<TaskInfo> {
		STORE.all()
	}
}

mod store;
mod types;

pub use types::TaskInfo;

use crate::{Error, Result};
use puniyu_types::task::TaskBuilder;
use std::sync::{Arc, LazyLock};
use store::TaskStore;

static STORE: LazyLock<TaskStore> = LazyLock::new(TaskStore::new);

pub struct TaskRegistry;

impl TaskRegistry {
	/// 注册任务
	///
	/// # 参数
	/// - `plugin_id`: 插件 ID
	/// - `builder`: 任务构建器
	///
	pub async fn register(plugin_id: u64, builder: Arc<dyn TaskBuilder>) -> Result<u64> {
		let task = TaskInfo { plugin_id, builder };
		STORE.insert(task).await
	}

	/// 卸载任务
	///
	/// # 参数
	/// - `task`: 任务 ID 或任务名称
	///
	pub async fn unregister<T>(task: T) -> Result<()>
	where
		T: Into<types::TaskId>,
	{
		let task = task.into();
		match task {
			types::TaskId::Index(id) => Self::unregister_with_index(id).await,
			types::TaskId::Name(name) => Self::unregister_with_task_name(&name).await,
		}
	}

	pub async fn unregister_with_index(index: u64) -> Result<()> {
		STORE.remove(index).await
	}

	pub async fn unregister_with_task_name(name: &str) -> Result<()> {
		let task_ids: Vec<u64> = {
			let raw = STORE.raw();
			let map = raw.read().expect("Failed to acquire lock");
			map.iter().filter(|(_, task)| task.builder.name() == name).map(|(id, _)| *id).collect()
		};

		if task_ids.is_empty() {
			return Err(Error::NotFound("Task".to_string()));
		}

		for id in task_ids {
			STORE.remove(id).await?;
		}
		Ok(())
	}

	pub async fn unregister_with_plugin_id(plugin_id: u64) -> Result<()> {
		let task_ids: Vec<u64> = {
			let raw = STORE.raw();
			let map = raw.read().expect("Failed to acquire lock");
			map.iter().filter(|(_, task)| task.plugin_id == plugin_id).map(|(id, _)| *id).collect()
		};

		if task_ids.is_empty() {
			return Err(Error::NotFound("Task".to_string()));
		}

		for id in task_ids {
			STORE.remove(id).await?;
		}
		Ok(())
	}

	pub fn get<T>(task: T) -> Vec<TaskInfo>
	where
		T: Into<types::TaskId>,
	{
		let task = task.into();
		match task {
			types::TaskId::Index(id) => Self::get_with_index(id).into_iter().collect(),
			types::TaskId::Name(name) => Self::get_with_task_name(&name),
		}
	}

	pub fn get_with_index(index: u64) -> Option<TaskInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	pub fn get_with_task_name(name: &str) -> Vec<TaskInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.builder.name() == name).cloned().collect()
	}

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

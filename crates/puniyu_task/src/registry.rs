use crate::error::Error;
use crate::types::build_job;
use crate::{TaskId, TaskInfo};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::JobScheduler;
use uuid::Uuid;

#[derive(Default)]
struct State {
	next_id: u64,
	tasks: HashMap<u64, TaskInfo>,
	job_ids: HashMap<u64, Uuid>,
	scheduler: Option<JobScheduler>,
}

#[derive(Clone, Default)]
pub struct TaskRegistry {
	inner: Arc<Mutex<State>>,
}

impl TaskRegistry {
	pub fn new() -> Self {
		Self::default()
	}

	/// 注册任务。调度器尚未启动时，任务会等待下一次启动统一装载。
	pub async fn register(
		&self,
		scope_id: u64,
		task: impl Into<Arc<dyn crate::Task>>,
	) -> Result<u64, Error> {
		let info = TaskInfo { scope_id, task: task.into() };
		let job = build_job(&info)?;
		let mut state = self.inner.lock().await;
		if state.tasks.values().any(|registered| {
			registered.scope_id == scope_id && registered.task.name() == info.task.name()
		}) {
			return Err(Error::Exists(format!("Task '{}' in scope {scope_id}", info.task.name())));
		}

		let id = state.next_id;
		state.next_id = state.next_id.checked_add(1).ok_or(Error::IdExhausted)?;
		if let Some(scheduler) = state.scheduler.as_ref() {
			let job_id =
				scheduler.add(job).await.map_err(|error| Error::Start(error.to_string()))?;
			state.job_ids.insert(id, job_id);
		}
		state.tasks.insert(id, info);
		Ok(id)
	}

	/// 卸载任务。
	pub async fn unregister<'t, T>(&self, task: T) -> Result<(), Error>
	where
		T: Into<TaskId<'t>>,
	{
		let task = task.into();
		match task {
			TaskId::Index(id) => self.unregister_with_index(id).await,
			TaskId::Name(name) => self.unregister_with_task_name(name.as_ref()).await,
		}
	}

	/// 通过索引卸载任务。
	pub async fn unregister_with_index(&self, index: u64) -> Result<(), Error> {
		let mut state = self.inner.lock().await;
		Self::remove_locked(&mut state, index).await
	}

	/// 通过任务名称卸载所有同名任务。
	pub async fn unregister_with_task_name(&self, name: &str) -> Result<(), Error> {
		let mut state = self.inner.lock().await;
		let task_ids: Vec<u64> = state
			.tasks
			.iter()
			.filter(|(_, info)| info.task.name() == name)
			.map(|(id, _)| *id)
			.collect();
		if task_ids.is_empty() {
			return Err(Error::NotFound(format!("Task '{name}'")));
		}
		for id in task_ids {
			Self::remove_locked(&mut state, id).await?;
		}
		Ok(())
	}

	/// 卸载指定插件作用域注册的全部任务。
	pub async fn remove_by_scope(&self, scope_id: u64) -> Result<(), Error> {
		let mut state = self.inner.lock().await;
		let task_ids: Vec<u64> = state
			.tasks
			.iter()
			.filter(|(_, info)| info.scope_id == scope_id)
			.map(|(id, _)| *id)
			.collect();
		for id in task_ids {
			Self::remove_locked(&mut state, id).await?;
		}
		Ok(())
	}

	/// 查询任务。
	pub async fn get<'t, T>(&self, task: T) -> Vec<TaskInfo>
	where
		T: Into<TaskId<'t>>,
	{
		let task = task.into();
		match task {
			TaskId::Index(id) => self.get_with_index(id).await.into_iter().collect(),
			TaskId::Name(name) => self.get_with_task_name(name.as_ref()).await,
		}
	}

	pub async fn get_with_index(&self, index: u64) -> Option<TaskInfo> {
		self.inner.lock().await.tasks.get(&index).cloned()
	}

	pub async fn get_with_task_name(&self, name: &str) -> Vec<TaskInfo> {
		self.inner
			.lock()
			.await
			.tasks
			.values()
			.filter(|info| info.task.name() == name)
			.cloned()
			.collect()
	}

	pub async fn get_by_scope(&self, scope_id: u64) -> Vec<TaskInfo> {
		self.inner
			.lock()
			.await
			.tasks
			.values()
			.filter(|info| info.scope_id == scope_id)
			.cloned()
			.collect()
	}

	pub async fn all(&self) -> Vec<TaskInfo> {
		self.inner.lock().await.tasks.values().cloned().collect()
	}

	/// 创建新的调度器实例并装载所有已登记任务。
	pub async fn start(&self) -> Result<(), Error> {
		let mut state = self.inner.lock().await;
		if state.scheduler.is_some() {
			return Ok(());
		}

		let mut scheduler =
			JobScheduler::new().await.map_err(|error| Error::Create(error.to_string()))?;
		let tasks: Vec<(u64, TaskInfo)> =
			state.tasks.iter().map(|(id, info)| (*id, info.clone())).collect();
		let mut job_ids = HashMap::with_capacity(tasks.len());
		for (id, info) in tasks {
			let job = build_job(&info)?;
			let job_id =
				scheduler.add(job).await.map_err(|error| Error::Start(error.to_string()))?;
			job_ids.insert(id, job_id);
		}
		if let Err(error) = scheduler.start().await {
			let _ = scheduler.shutdown().await;
			return Err(Error::Start(error.to_string()));
		}
		state.job_ids = job_ids;
		state.scheduler = Some(scheduler);
		Ok(())
	}

	/// 停止并丢弃当前调度器，保留任务定义供下次重新启动。
	pub async fn stop(&self) -> Result<(), Error> {
		let mut scheduler = {
			let mut state = self.inner.lock().await;
			state.job_ids.clear();
			state.scheduler.take()
		};
		if let Some(scheduler) = scheduler.as_mut() {
			scheduler.shutdown().await.map_err(|error| Error::Shutdown(error.to_string()))?;
		}
		Ok(())
	}

	pub async fn is_running(&self) -> bool {
		self.inner.lock().await.scheduler.is_some()
	}

	async fn remove_locked(state: &mut State, id: u64) -> Result<(), Error> {
		if !state.tasks.contains_key(&id) {
			return Err(Error::NotFound(format!("Task index {id}")));
		}
		if let Some(job_id) = state.job_ids.get(&id).copied() {
			if let Some(scheduler) = state.scheduler.as_ref() {
				scheduler
					.remove(&job_id)
					.await
					.map_err(|error| Error::Shutdown(error.to_string()))?;
			}
			state.job_ids.remove(&id);
		}
		state.tasks.remove(&id);
		Ok(())
	}
}

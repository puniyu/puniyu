use crate::error::Error;
use crate::scheduler::build_job;
use crate::{Task, TaskId};
use parking_lot::Mutex;
use std::sync::{Arc, LazyLock};
use tokio_cron_scheduler::JobScheduler;
use uuid::Uuid;


/// 定时任务注册表。
///
/// 基于无锁并发 [`Registry`](puniyu_registry::Registry) 存储任务，
/// 支持在调度器运行时动态注册/卸载任务。
///
/// # 示例
///
/// ```rust
/// # use std::sync::Arc;
/// # use puniyu_task::{Task, TaskRegistry};
/// # use async_trait::async_trait;
/// # use puniyu_error::AnyError;
/// # struct MyTask;
/// # #[async_trait]
/// # impl Task for MyTask {
/// #     fn name(&self) -> &str { "my_task" }
/// #     fn cron(&self) -> &str { "0 * * * * *" }
/// #     async fn execute(&self) -> AnyError { Ok(()) }
/// # }
/// # tokio_test::block_on(async {
/// let registry = TaskRegistry::new();
/// let task: Arc<dyn Task> = Arc::new(MyTask);
/// registry.insert(task).await.unwrap();
/// # })
/// ```
#[derive(Clone)]
pub struct TaskRegistry {
	tasks: puniyu_registry::Registry<Arc<dyn Task>>,
	job_ids: Arc<scc::HashMap<u64, Uuid>>,
	scheduler: Arc<Mutex<Option<JobScheduler>>>,
}

impl Default for TaskRegistry {
	fn default() -> Self {
		static TASKS: LazyLock<puniyu_registry::Registry<Arc<dyn Task>>> =
			LazyLock::new(puniyu_registry::Registry::new);
		static JOB_IDS: LazyLock<Arc<scc::HashMap<u64, Uuid>>> =
			LazyLock::new(|| Arc::new(scc::HashMap::new()));
		static SCHEDULER: LazyLock<Arc<Mutex<Option<JobScheduler>>>> =
			LazyLock::new(|| Arc::new(Mutex::new(None)));
		Self { tasks: TASKS.clone(), job_ids: JOB_IDS.clone(), scheduler: SCHEDULER.clone() }
	}
}

impl TaskRegistry {
	pub fn new() -> Self {
		Self::default()
	}

	/// 注册任务，返回分配的任务 ID。
	///
	/// 如果调度器正在运行，任务会立即加入调度器。
	/// 同名任务不允许重复注册。
	pub async fn insert(&self, task: impl Into<Arc<dyn Task>>) -> Result<u64, Error> {
		let task = task.into();
		let id = puniyu_registry::Registry::insert(&self.tasks, task.clone());
		let scheduler_ref = self.scheduler.lock().clone();
		if let Some(scheduler) = scheduler_ref {
			let job = build_job(&task)?;
			let job_id = scheduler.add(job).await.map_err(|e| Error::InvalidSchedule {
				task: task.name().to_string(),
				message: e.to_string(),
			})?;
			self.job_ids.insert_sync(id, job_id).ok();
		}

		Ok(id)
	}

	/// 卸载任务。
	pub async fn remove<'t, T>(&self, task: T) -> Result<(), Error>
	where
		T: Into<TaskId<'t>>,
	{
		match task.into() {
			TaskId::Index(id) => self.remove_with_index(id).await,
			TaskId::Name(name) => self.remove_with_name(name.as_ref()).await,
		}
	}

	/// 通过索引卸载任务。
	pub async fn remove_with_index(&self, id: u64) -> Result<(), Error> {
		self.tasks.remove(id);
		if let Some((_, job_id)) = self.job_ids.remove_sync(&id) {
			let scheduler = self.scheduler.lock().clone();
			if let Some(scheduler) = scheduler {
				scheduler.remove(&job_id).await.map_err(|e| Error::InvalidSchedule {
					task: format!("index {id}"),
					message: e.to_string(),
				})?;
			}
		}
		Ok(())
	}

	/// 通过任务名称卸载所有同名任务。
	pub async fn remove_with_name(&self, name: &str) -> Result<(), Error> {
		let ids: Vec<u64> = self
			.tasks
			.iter()
			.into_iter()
			.filter(|(_, task)| task.name() == name)
			.map(|(id, _)| id)
			.collect();

		let scheduler_ref = self.scheduler.lock().clone();
		for id in ids {
			self.tasks.remove(id);
			if let Some((_, job_id)) = self.job_ids.remove_sync(&id)
				&& let Some(ref scheduler) = scheduler_ref
			{
				scheduler.remove(&job_id).await.map_err(|e| Error::InvalidSchedule {
					task: name.to_string(),
					message: e.to_string(),
				})?;
			}
		}
		Ok(())
	}

	pub fn get<'t, T>(&self, task: T) -> Vec<Arc<dyn Task>>
	where
		T: Into<TaskId<'t>>,
	{
		match task.into() {
			TaskId::Index(id) => self.get_with_index(id).into_iter().collect(),
			TaskId::Name(name) => self.get_with_name(name.as_ref()),
		}
	}

	pub fn get_with_index(&self, index: u64) -> Option<Arc<dyn Task>> {
		self.tasks.get(index)
	}

	pub fn get_with_name(&self, name: &str) -> Vec<Arc<dyn Task>> {
		self.tasks.iter().into_iter().filter(|(_, t)| t.name() == name).map(|(_, t)| t).collect()
	}

	pub fn values(&self) -> Vec<Arc<dyn Task>> {
		self.tasks.values()
	}

	pub async fn start(&self) -> Result<(), Error> {
		{
			let guard = self.scheduler.lock();
			if guard.is_some() {
				return Ok(());
			}
		}

		let mut scheduler = JobScheduler::new().await.map_err(|e| Error::InvalidSchedule {
			task: String::new(),
			message: format!("create scheduler: {e}"),
		})?;

		let tasks = self.tasks.iter();
		for (id, task) in tasks {
			let job = build_job(&task)?;
			let job_id = scheduler.add(job).await.map_err(|e| Error::InvalidSchedule {
				task: task.name().to_string(),
				message: e.to_string(),
			})?;
			self.job_ids.insert_sync(id, job_id).ok();
		}

		if let Err(e) = scheduler.start().await {
			let _ = scheduler.shutdown().await;
			return Err(Error::InvalidSchedule {
				task: String::new(),
				message: format!("start scheduler: {e}"),
			});
		}

		*self.scheduler.lock() = Some(scheduler);
		Ok(())
	}

	/// 停止并丢弃当前调度器，保留任务定义供下次重新启动。
	pub async fn stop(&self) -> Result<(), Error> {
		let mut scheduler = self.scheduler.lock().take();
		self.job_ids.clear_sync();
		if let Some(ref mut s) = scheduler {
			s.shutdown().await.map_err(|e| Error::InvalidSchedule {
				task: String::new(),
				message: format!("shutdown scheduler: {e}"),
			})?;
		}
		Ok(())
	}

	/// 调度器是否正在运行
	pub fn is_running(&self) -> bool {
		self.scheduler.lock().is_some()
	}
}

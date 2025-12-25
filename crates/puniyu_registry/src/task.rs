use chrono_tz::Asia::Shanghai;
use puniyu_logger::{info, error, owo_colors::OwoColorize};
use puniyu_types::task::{TaskBuilder, TaskId};
use std::time::Instant;
use std::{
	collections::HashMap,
	sync::{Arc, LazyLock, RwLock},
};
use tokio::sync::OnceCell;
use tokio_cron_scheduler::{JobBuilder, JobScheduler};
use uuid::Uuid;


#[derive(Clone)]
struct Task {
	plugin_name: String,
	builder: Arc<dyn TaskBuilder>,
}

impl From<Task> for tokio_cron_scheduler::Job {
	fn from(task: Task) -> Self {
		JobBuilder::new()
			.with_timezone(Shanghai)
			.with_cron_job_type()
			.with_schedule(task.builder.cron())
			.unwrap()
			.with_run_async(Box::new({
				let task_name = task.builder.name().to_string();
				let plugin_name = task.plugin_name.clone();
				let builder = task.builder.clone();
				move |_uuid, _lock| {
					let task_name = task_name.clone();
					let plugin_name = plugin_name.clone();
					let builder = builder.clone();
					Box::pin(async move {
						let tag_str = format!("[task:{}:{}]", plugin_name, task_name);
						let tag = tag_str.fg_rgb::<255, 192, 203>();
						info!("{} 开始执行", tag);

						let start_time = Instant::now();
						let result = builder.run().await;
						let duration = start_time.elapsed().as_millis();

						match result {
							Ok(_) => info!("{} 执行完成,耗时: {}ms", tag, duration),
							Err(e) => error!("{} 执行失败,耗时: {}ms, 错误: {}", tag, duration, e),
						}
					})
				}
			}))
			.build()
			.unwrap()
	}
}

type TaskStore = Arc<RwLock<HashMap<u64, Arc<Task>>>>;
type TaskManager = Arc<RwLock<HashMap<u64, Uuid>>>;

static TASK_STORE: LazyLock<TaskStore> = LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

static TASK_MANAGER: LazyLock<TaskManager> =
	LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

static SCHEDULER: OnceCell<Arc<JobScheduler>> = OnceCell::const_new();

static TASK_ID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub struct TaskRegistry;

impl TaskRegistry {
	async fn get_scheduler() -> &'static Arc<JobScheduler> {
		SCHEDULER
			.get_or_init(|| async {
				let sched = JobScheduler::new().await.unwrap();
				Arc::new(sched)
			})
			.await
	}

	/// 添加任务
	///
	/// # 参数
	/// - `plugin_name`: 插件名称
	/// - `builder`: 任务构建器
	///
	pub async fn add_task(plugin_name: &str, builder: Arc<dyn TaskBuilder>) {
		let id = TASK_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

		let task = Arc::new(Task { plugin_name: plugin_name.to_string(), builder });
		TASK_STORE.write().unwrap().insert(id, task.clone());

		let job = tokio_cron_scheduler::Job::from(Task {
			plugin_name: task.plugin_name.clone(),
			builder: task.builder.clone(),
		});

		let scheduler = Self::get_scheduler().await;
		let uuid = scheduler.add(job).await.unwrap();

		TASK_MANAGER.write().unwrap().insert(id, uuid);
	}

	/// 移除任务
	///
	/// # 参数
	/// - `task`: 任务 ID 或插件名称
	///
	pub async fn remove_task(task: impl Into<TaskId>) {
		match task.into() {
			TaskId::Index(id) => Self::remove_by_id(id).await,
			TaskId::Name(plugin_name) => Self::remove_by_plugin(&plugin_name).await,
		}
	}

	/// 按 ID 移除任务
	async fn remove_by_id(id: u64) {
		let uuid = TASK_MANAGER.write().unwrap().remove(&id);
		TASK_STORE.write().unwrap().remove(&id);

		if let Some(uuid) = uuid {
			let scheduler = Self::get_scheduler().await;
			let _ = scheduler.remove(&uuid).await;
		}
	}

	/// 按插件名移除所有任务
	async fn remove_by_plugin(plugin_name: &str) {
		let task_ids: Vec<u64> = {
			let store = TASK_STORE.read().unwrap();
			store
				.iter()
				.filter(|(_, task)| task.plugin_name == plugin_name)
				.map(|(id, _)| *id)
				.collect()
		};

		for id in task_ids {
			Self::remove_by_id(id).await;
		}
	}

	/// 获取所有任务
	pub fn get_all_tasks() -> Vec<puniyu_types::task::Task> {
		let store = TASK_STORE.read().unwrap();
		store
			.values()
			.map(|task| puniyu_types::task::Task {
				name: task.builder.name(),
				cron: task.builder.cron(),
			})
			.collect()
	}

	/// 获取特定任务信息
	///
	/// # 参数
	/// - `task`: 任务 ID 或插件名称
	///
	pub fn get_task(task: impl Into<TaskId>) -> Option<puniyu_types::task::Task> {
		let store = TASK_STORE.read().unwrap();
		match task.into() {
			TaskId::Index(id) => store.get(&id).map(|task| puniyu_types::task::Task {
				name: task.builder.name(),
				cron: task.builder.cron(),
			}),
			TaskId::Name(name) => store
				.values()
				.find(|task| task.builder.name() == name.as_str())
				.map(|task| puniyu_types::task::Task {
					name: task.builder.name(),
					cron: task.builder.cron(),
				}),
		}
	}
}

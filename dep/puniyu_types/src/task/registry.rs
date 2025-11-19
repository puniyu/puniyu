use super::TaskId;
use super::builder::TaskBuilder;
use chrono_tz::Asia::Shanghai;
use puniyu_logger::{info, owo_colors::OwoColorize};
use std::{
	collections::HashMap,
	sync::{Arc, LazyLock, RwLock},
	time::Instant,
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
			.with_run_async(Box::new(move |_uuid, _lock| {
				let builder = task.builder.clone();
				Box::pin(async move {
					let name = builder.name().to_string();
					let task_run = builder.run();
					let start_time = Instant::now();
					let prefix = "task".fg_rgb::<176, 196, 222>();
					let message = name.fg_rgb::<255, 192, 203>();
					info!("[{}:{}] 开始执行", prefix, message);

					task_run.await;

					let duration = start_time.elapsed().as_millis();
					info!("[{}:{}] 执行完成,耗时: {}ms", prefix, message, duration);
				})
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
	pub fn get_all_tasks() -> Vec<super::Task> {
		let store = TASK_STORE.read().unwrap();
		store
			.values()
			.map(|task| super::Task { name: task.builder.name(), cron: task.builder.cron() })
			.collect()
	}

	/// 获取特定任务信息
	///
	/// # 参数
	/// - `task`: 任务 ID 或插件名称
	///
	pub fn get_task(task: impl Into<TaskId>) -> Option<super::Task> {
		let store = TASK_STORE.read().unwrap();
		match task.into() {
			TaskId::Index(id) => store
				.get(&id)
				.map(|task| super::Task { name: task.builder.name(), cron: task.builder.cron() }),
			TaskId::Name(name) => store
				.values()
				.find(|task| task.builder.name() == name.as_str())
				.map(|task| super::Task { name: task.builder.name(), cron: task.builder.cron() }),
		}
	}
}

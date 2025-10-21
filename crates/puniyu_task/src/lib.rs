use chrono_tz::Asia::Shanghai;

use puniyu_builder::task::TaskBuilder;
use puniyu_logger::info;
use puniyu_logger::owo_colors::OwoColorize;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::OnceCell;
pub use tokio_cron_scheduler;
use tokio_cron_scheduler::{JobBuilder, JobScheduler};
pub use uuid::Uuid;
pub static SCHEDULER: OnceCell<Arc<JobScheduler>> = OnceCell::const_new();

pub struct Task {
	/// 插件名称
	pub plugin_name: &'static str,
	/// 任务构建器
	pub builder: Arc<dyn TaskBuilder>,
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
					info!("[{}:{}] 执行完成，耗时: {}ms", prefix, message, duration);
				})
			}))
			.build()
			.unwrap()
	}
}

pub async fn init_scheduler() {
	SCHEDULER
		.get_or_init(|| async {
			let sched = JobScheduler::new().await.unwrap();
			Arc::new(sched)
		})
		.await;
}

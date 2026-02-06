use chrono_tz::Asia::Shanghai;
use puniyu_logger::owo_colors::OwoColorize;
use puniyu_logger::{error, info};
use puniyu_types::task::TaskBuilder;
use std::sync::Arc;
use std::time::Instant;
use tokio_cron_scheduler::JobBuilder;

#[derive(Clone)]
pub struct TaskInfo {
	pub plugin_id: u64,
	pub builder: Arc<dyn TaskBuilder>,
}

impl From<TaskInfo> for tokio_cron_scheduler::Job {
	fn from(task: TaskInfo) -> Self {
		JobBuilder::new()
			.with_timezone(Shanghai)
			.with_cron_job_type()
			.with_schedule(task.builder.cron())
			.unwrap()
			.with_run_async(Box::new({
				let task_name = task.builder.name().to_string();
				let builder = task.builder.clone();
				move |_uuid, _lock| {
					let task_name = task_name.clone();
					let builder = builder.clone();
					Box::pin(async move {
						let tag_str = format!("[task:{}]", task_name);
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
			.expect("Failed to create job")
	}
}

#[derive(Debug, Clone)]
pub enum TaskId {
	Index(u64),
	Name(String),
}

impl From<u64> for TaskId {
	fn from(value: u64) -> Self {
		Self::Index(value)
	}
}
impl From<String> for TaskId {
	fn from(value: String) -> Self {
		Self::Name(value)
	}
}

impl From<&str> for TaskId {
	fn from(value: &str) -> Self {
		Self::Name(value.to_string())
	}
}

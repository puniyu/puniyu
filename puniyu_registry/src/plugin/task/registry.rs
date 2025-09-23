use super::builder::TaskBuilder;
use crate::logger::info;
use crate::logger::owo_colors::OwoColorize;
use chrono_tz::Asia::Shanghai;
use std::sync::Arc;
use std::time::Instant;
use tokio_cron_scheduler::JobBuilder;

pub struct TaskRegistry {
	/// 插件名称
	pub plugin_name: &'static str,
	/// 任务构建器
	pub builder: Arc<dyn TaskBuilder>,
}

impl From<TaskRegistry> for tokio_cron_scheduler::Job {
	fn from(value: TaskRegistry) -> Self {
		JobBuilder::new()
			.with_timezone(Shanghai)
			.with_cron_job_type()
			.with_schedule(value.builder.cron())
			.unwrap()
			.with_run_async(Box::new(move |_uuid, _lock| {
				let name = value.builder.name().to_string();
				let task_run = value.builder.run();
				Box::pin(async move {
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

use super::builder::TaskBuilder;
use crate::logger::info;
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
					info!("[定时计划:{}] 开始执行", name);

					task_run.await;

					let duration = start_time.elapsed().as_millis();
					info!("[定时计划:{}] 执行完成，耗时: {}ms", name, duration);
				})
			}))
			.build()
			.unwrap()
	}
}

use crate::Task;
use std::borrow::Cow;
use std::sync::Arc;
use std::time::Instant;
use tokio_cron_scheduler::JobBuilder;

macro_rules! info {
    ($name:expr, $($arg:tt)*) => {
        {
            {
                use ::puniyu_logger::owo_colors::OwoColorize;
                let tag = format!("Task:{}", $name);
                let prefix = tag.fg_rgb::<255, 192, 203>();
                ::log::info!("[{}] {}", prefix, format!($($arg)*))
            }
        }
    };
}
macro_rules! error {
    ($name:expr, $($arg:tt)*) => {
        {
            {
                use ::puniyu_logger::owo_colors::OwoColorize;
                let tag = format!("Task:{}", $name);
                let prefix = tag.fg_rgb::<255, 192, 203>();
                ::log::error!("[{}] {}", prefix, format!($($arg)*))
            }
        }
    };
}

/// 任务信息
#[derive(Clone)]
pub struct TaskInfo {
	/// 关联的插件 ID
	pub plugin_id: u64,
	/// 任务
	pub task: Arc<dyn Task>,
}

impl From<&TaskInfo> for tokio_cron_scheduler::Job {
	fn from(info: &TaskInfo) -> Self {
		let task = info.task.clone();
		let cron_str = task.cron().to_string();
		let task_name = task.name().to_string();
		JobBuilder::new()
			.with_timezone(chrono::Local)
			.with_cron_job_type()
			.with_schedule(&cron_str)
			.expect("Invalid cron schedule")
			.with_run_async(Box::new(move |_uuid, _lock| {
				let task_name = task_name.clone();
				let task = task.clone();
				Box::pin(async move {
					info!(task_name, "开始执行");

					let start_time = Instant::now();
					let result = task.execute().await;
					let duration = start_time.elapsed().as_millis();

					match result {
						Ok(_) => info!(task_name, "执行完成,耗时: {}ms", duration),
						Err(e) => error!(task_name, "执行失败,耗时: {}ms, 错误: {}", duration, e),
					}
				})
			}))
			.build()
			.expect("Failed to create job")
	}
}

impl From<TaskInfo> for tokio_cron_scheduler::Job {
	#[inline]
	fn from(task: TaskInfo) -> Self {
		tokio_cron_scheduler::Job::from(&task)
	}
}

/// 任务 ID 枚举
#[derive(Debug, Clone)]
pub enum TaskId<'t> {
	/// 任务索引 ID
	Index(u64),
	/// 任务名称
	Name(Cow<'t, str>),
}

impl From<u64> for TaskId<'_> {
	#[inline]
	fn from(value: u64) -> Self {
		Self::Index(value)
	}
}

impl<'t> From<&'t str> for TaskId<'t> {
	#[inline]
	fn from(value: &'t str) -> Self {
		Self::Name(Cow::Borrowed(value))
	}
}

impl From<String> for TaskId<'_> {
	#[inline]
	fn from(value: String) -> Self {
		Self::Name(Cow::Owned(value))
	}
}

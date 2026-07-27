use std::sync::Arc;
use std::time::Instant;

use crate::error::Error;
use crate::Task;
use tokio_cron_scheduler::{Job, JobBuilder};

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

pub(crate) fn build_job(task: &Arc<dyn Task>) -> Result<Job, Error> {
    let task = task.clone();
    let cron_str = task.cron().to_string();
    let task_name = task.name().to_string();
    let job = JobBuilder::new()
        .with_timezone(chrono::Local)
        .with_cron_job_type()
        .with_schedule(&cron_str)
        .map_err(|error| Error::InvalidSchedule {
            task: task_name.clone(),
            message: error.to_string(),
        })?
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
        .map_err(|error| Error::InvalidSchedule {
            task: cron_str,
            message: error.to_string(),
        })?;
    Ok(job)
}

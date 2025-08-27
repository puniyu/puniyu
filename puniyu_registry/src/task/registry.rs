use super::TaskInfo;
use chrono_tz::Asia::Shanghai;
use log::info;
use std::time::Instant;
use tokio_cron_scheduler::{JobBuilder, JobScheduler};

#[derive(Clone)]
pub struct TaskRegistry {
    /// 任务实例
    pub task: &'static dyn TaskInfo,
}

inventory::collect!(TaskRegistry);

/// 这个需要初始化
pub async fn init_task() -> JobScheduler {
    let scheduler = JobScheduler::new().await.unwrap();

    let job_futures: Vec<_> = inventory::iter::<TaskRegistry>
        .into_iter()
        .map(|task_registry| {
            let task = task_registry.task;
            let job = JobBuilder::new()
                .with_timezone(Shanghai)
                .with_schedule(task.cron())
                .unwrap()
                .with_run_async(Box::new(move |_uuid, _lock| {
                    let name = task.name();
                    Box::pin(async move {
                        let start_time = Instant::now();
                        info!("[定时计划:{}] 开始执行", name);

                        task.run().await;

                        let duration = start_time.elapsed().as_millis();
                        info!("[定时计划:{}] 执行完成，耗时: {}ms", name, duration);
                    })
                }))
                .build()
                .unwrap();
            scheduler.add(job)
        })
        .collect();
    futures::future::join_all(job_futures).await;

    scheduler.start().await.unwrap();
    info!("定时任务初始化完成");
    scheduler
}

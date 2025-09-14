use crate::plugin::task::builder::TaskBuilder;
use chrono_tz::Asia::Shanghai;
use hashbrown::HashMap;
use log::info;
use std::{
    sync::{LazyLock, Mutex},
    time::Instant,
};
use tokio_cron_scheduler::{JobBuilder, JobScheduler};

static TASK_REGISTRY: LazyLock<Mutex<HashMap<String, Box<dyn TaskBuilder>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub struct TaskManager;

impl TaskManager {
    pub fn register_task(task: Box<dyn TaskBuilder>) {
        let mut registry = TASK_REGISTRY.lock().unwrap();
        registry.insert(task.name().to_string(), task);
    }

    pub(crate) async fn init_scheduler() -> () {
        let scheduler = JobScheduler::new().await.unwrap();
        scheduler.start().await.unwrap();

        let tasks: Vec<Box<dyn TaskBuilder>> = {
            let mut registry = TASK_REGISTRY.lock().unwrap();
            std::mem::take(&mut *registry).into_values().collect()
        };

        for task in tasks {
            let job = JobBuilder::new()
                .with_timezone(Shanghai)
                .with_cron_job_type()
                .with_schedule(task.cron())
                .unwrap()
                .with_run_async(Box::new(move |_uuid, _lock| {
                    let name = task.name().to_string();
                    let task_run = task.run();
                    Box::pin(async move {
                        let start_time = Instant::now();
                        info!("[定时计划:{}] 开始执行", name);

                        task_run.await;

                        let duration = start_time.elapsed().as_millis();
                        info!("[定时计划:{}] 执行完成，耗时: {}ms", name, duration);
                    })
                }))
                .build()
                .unwrap();

            scheduler.add(job).await.unwrap();
        }
    }
}

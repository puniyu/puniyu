use chrono_tz::Asia::Shanghai;
use log::info;
use std::{future::Future, pin::Pin, time::Instant};
use tokio_cron_scheduler::{Job, JobBuilder, JobScheduler};
type TaskFn = fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
#[derive(Debug, Clone)]
pub struct TaskRegistry {
    /// 定时计划名称
    pub name: &'static str,
    /// cron表达式
    pub cron: &'static str,
    /// 任务函数
    pub func: TaskFn,
}

inventory::collect!(TaskRegistry);

/// 注册异步定时任务的宏
#[macro_export]
macro_rules! register_task {
    ($name:expr, $cron:expr, $func:expr) => {
        const _: () = {
            inventory::submit! {
                $crate::registry::TaskRegistry {
                    name: $name,
                    cron: $cron,
                    func: || ::std::boxed::Box::pin(async move {
                        if let Err(_e) = $func().await {
                            log::error!("定时任务 {} 执行失败", $name);
                        }
                    }),
                }
            }
        };
    };
}

/// 这个需要初始化
pub async fn init_task() -> JobScheduler {
    let scheduler = JobScheduler::new().await.unwrap();

    for task in inventory::iter::<TaskRegistry> {
        let job = JobBuilder::new()
            .with_timezone(Shanghai)
            .with_run_async(Box::new(move |_uuid, _lock| {
                let name = task.name;
                let fnc = task.func;
                Box::pin(async move {
                    let start_time = Instant::now();
                    info!("[定时计划:{}] 开始执行", name);

                    fnc().await;

                    let duration = start_time.elapsed().as_millis();
                    info!("[定时计划:{}] 执行完成，耗时: {}ms", name, duration);
                })
            }))
            .build()
            .unwrap();
        scheduler.add(job).await.unwrap();
    }

    scheduler.start().await.unwrap();
    info!("定时任务初始化完成");
    scheduler
}

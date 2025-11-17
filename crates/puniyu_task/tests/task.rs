use async_trait::async_trait;
use puniyu_builder::task::TaskBuilder;
use puniyu_task::{init_scheduler, Task, SCHEDULER};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

// 简单任务
struct SimpleTask {
    name: &'static str,
    cron: &'static str,
    executed: Arc<AtomicBool>,
}

#[async_trait]
impl TaskBuilder for SimpleTask {
    fn name(&self) -> &'static str {
        self.name
    }

    fn cron(&self) -> &'static str {
        self.cron
    }

    async fn run(&self) {
        self.executed.store(true, Ordering::SeqCst);
    }
}

/// 计数任务
struct CounterTask {
    name: &'static str,
    cron: &'static str,
    counter: Arc<AtomicU32>,
}

#[async_trait]
impl TaskBuilder for CounterTask {
    fn name(&self) -> &'static str {
        self.name
    }

    fn cron(&self) -> &'static str {
        self.cron
    }

    async fn run(&self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
    }
}

// 延迟任务
struct DelayTask {
    name: &'static str,
    cron: &'static str,
    delay_ms: u64,
}

#[async_trait]
impl TaskBuilder for DelayTask {
    fn name(&self) -> &'static str {
        self.name
    }

    fn cron(&self) -> &'static str {
        self.cron
    }

    async fn run(&self) {
        sleep(Duration::from_millis(self.delay_ms)).await;
    }
}

#[tokio::test]
async fn test_create_task() {
    let executed = Arc::new(AtomicBool::new(false));
    let task_builder = SimpleTask {
        name: "test_task",
        cron: "0 * * * * *",
        executed: executed.clone(),
    };

    let task = Task {
        plugin_name: "test_plugin",
        builder: Arc::new(task_builder),
    };

    assert_eq!(task.plugin_name, "test_plugin");
    assert_eq!(task.builder.name(), "test_task");
    assert_eq!(task.builder.cron(), "0 * * * * *");
}

#[tokio::test]
async fn test_builder_name() {
    let executed = Arc::new(AtomicBool::new(false));
    let builder = SimpleTask {
        name: "my_task",
        cron: "0 0 * * * *",
        executed,
    };

    assert_eq!(builder.name(), "my_task");
}

#[tokio::test]
async fn test_builder_cron() {
    let executed = Arc::new(AtomicBool::new(false));
    let builder = SimpleTask {
        name: "test",
        cron: "0 30 * * * *",
        executed,
    };

    assert_eq!(builder.cron(), "0 30 * * * *");
}

#[tokio::test]
async fn test_builder_run() {
    let executed = Arc::new(AtomicBool::new(false));
    let builder = SimpleTask {
        name: "test",
        cron: "0 * * * * *",
        executed: executed.clone(),
    };

    assert!(!executed.load(Ordering::SeqCst));
    builder.run().await;
    assert!(executed.load(Ordering::SeqCst));
}

#[tokio::test]
async fn test_counter() {
    let counter = Arc::new(AtomicU32::new(0));
    let builder = CounterTask {
        name: "counter",
        cron: "0 * * * * *",
        counter: counter.clone(),
    };

    assert_eq!(counter.load(Ordering::SeqCst), 0);
    builder.run().await;
    assert_eq!(counter.load(Ordering::SeqCst), 1);
    builder.run().await;
    assert_eq!(counter.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn test_init() {
    init_scheduler().await;
    assert!(SCHEDULER.get().is_some());
}

#[tokio::test]
async fn test_init_multiple() {
    init_scheduler().await;
    init_scheduler().await;
    init_scheduler().await;
    
    assert!(SCHEDULER.get().is_some());
}

#[tokio::test]
async fn test_to_job() {
    let executed = Arc::new(AtomicBool::new(false));
    let task_builder = SimpleTask {
        name: "conversion_test",
        cron: "0 * * * * *",
        executed: executed.clone(),
    };

    let task = Task {
        plugin_name: "test_plugin",
        builder: Arc::new(task_builder),
    };

    let _job = tokio_cron_scheduler::Job::from(task);
}

#[tokio::test]
async fn test_multiple() {
    let executed1 = Arc::new(AtomicBool::new(false));
    let executed2 = Arc::new(AtomicBool::new(false));
    let executed3 = Arc::new(AtomicBool::new(false));

    let builder1 = SimpleTask {
        name: "task1",
        cron: "0 * * * * *",
        executed: executed1.clone(),
    };

    let builder2 = SimpleTask {
        name: "task2",
        cron: "0 30 * * * *",
        executed: executed2.clone(),
    };

    let builder3 = SimpleTask {
        name: "task3",
        cron: "0 0 * * * *",
        executed: executed3.clone(),
    };

    let task1 = Task {
        plugin_name: "plugin1",
        builder: Arc::new(builder1),
    };

    let task2 = Task {
        plugin_name: "plugin2",
        builder: Arc::new(builder2),
    };

    let task3 = Task {
        plugin_name: "plugin3",
        builder: Arc::new(builder3),
    };

    assert_eq!(task1.plugin_name, "plugin1");
    assert_eq!(task2.plugin_name, "plugin2");
    assert_eq!(task3.plugin_name, "plugin3");
}

#[tokio::test]
async fn test_cron_patterns() {
    let patterns = vec![
        "0 * * * * *",           // 每分钟
        "0 0 * * * *",           // 每小时
        "0 0 0 * * *",           // 每天
        "0 30 9 * * MON-FRI",    // 工作日上午9:30
        "0 0 12 * * SAT,SUN",    // 周末中午12:00
    ];

    for (i, pattern) in patterns.iter().enumerate() {
        let executed = Arc::new(AtomicBool::new(false));
        let builder = SimpleTask {
            name: "pattern_test",
            cron: pattern,
            executed: executed.clone(),
        };

        let task = Task {
            plugin_name: "test_plugin",
            builder: Arc::new(builder),
        };

        assert_eq!(task.builder.cron(), *pattern, "Pattern {} failed", i);
    }
}

#[tokio::test]
async fn test_delay() {
    let builder = DelayTask {
        name: "delay_task",
        cron: "0 * * * * *",
        delay_ms: 100,
    };

    let start = std::time::Instant::now();
    builder.run().await;
    let elapsed = start.elapsed();

    assert!(elapsed.as_millis() >= 100);
}

#[tokio::test]
async fn test_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    let executed = Arc::new(AtomicBool::new(false));
    let builder = SimpleTask {
        name: "test",
        cron: "0 * * * * *",
        executed,
    };

    assert_send::<SimpleTask>();
    assert_sync::<SimpleTask>();
    assert_send::<Arc<dyn TaskBuilder>>();
    assert_sync::<Arc<dyn TaskBuilder>>();

    let builder_arc = Arc::new(builder);
    let builder_clone = builder_arc.clone();
    
    tokio::spawn(async move {
        let _ = builder_clone.name();
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn test_static_name() {
    let executed = Arc::new(AtomicBool::new(false));
    let builder = SimpleTask {
        name: "test",
        cron: "0 * * * * *",
        executed,
    };

    let task = Task {
        plugin_name: "static_plugin",
        builder: Arc::new(builder),
    };

    let name: &'static str = task.plugin_name;
    assert_eq!(name, "static_plugin");
}

#[tokio::test]
async fn test_arc_clone() {
    let counter = Arc::new(AtomicU32::new(0));
    let builder = CounterTask {
        name: "clone_test",
        cron: "0 * * * * *",
        counter: counter.clone(),
    };

    let arc_builder = Arc::new(builder);
    let clone1 = arc_builder.clone();
    let clone2 = arc_builder.clone();

    arc_builder.run().await;
    assert_eq!(counter.load(Ordering::SeqCst), 1);

    clone1.run().await;
    assert_eq!(counter.load(Ordering::SeqCst), 2);

    clone2.run().await;
    assert_eq!(counter.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_concurrent() {
    let counter = Arc::new(AtomicU32::new(0));
    let builder = Arc::new(CounterTask {
        name: "concurrent_test",
        cron: "0 * * * * *",
        counter: counter.clone(),
    });
    let mut handles = vec![];
    for _ in 0..10 {
        let builder_clone = builder.clone();
        handles.push(tokio::spawn(async move {
            builder_clone.run().await;
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    assert_eq!(counter.load(Ordering::SeqCst), 10);
}

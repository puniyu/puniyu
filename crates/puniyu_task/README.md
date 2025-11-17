# puniyu_task

定时任务管理模块

## 概述

`puniyu_task` 是 puniyu 项目中负责定时任务调度和管理的核心库。它基于 `tokio-cron-scheduler` 实现，提供了任务的创建、调度和执行功能，支持 cron 表达式定时执行任务。

## 核心组件

### Task 结构体

任务包装结构体，用于将 `TaskBuilder` 转换为可调度的任务：

```rust
pub struct Task {
    pub plugin_name: &'static str,           // 插件名称
    pub builder: Arc<dyn TaskBuilder>,       // 任务构建器
}
```

**关键特性**：
- 实现了 `From<Task> for tokio_cron_scheduler::Job` trait，自动将任务转换为调度器可执行的 Job
- 自动包装任务执行逻辑，添加日志记录和性能监控
- 使用 `Arc` 实现线程安全的任务共享

### TaskBuilder Trait

定义了定时任务的标准接口（由 `puniyu_builder` 提供）：

```rust
#[async_trait]
pub trait TaskBuilder: Send + Sync + 'static {
    fn name(&self) -> &'static str;        // 任务名称
    fn cron(&self) -> &'static str;        // cron 表达式
    async fn run(&self);                   // 执行任务
}
```

### 全局调度器

- `SCHEDULER`: 使用 `OnceCell` 实现的全局任务调度器实例，保证线程安全的单例模式
- `init_scheduler()`: 初始化全局调度器的异步函数，使用 `get_or_init` 确保只初始化一次

## 功能特性

### 任务调度

- 基于 cron 表达式的定时任务调度
- 使用 `Asia/Shanghai` 时区
- 支持异步任务执行
- 自动记录任务执行时间和性能指标

### 任务执行日志

- 任务开始执行时记录日志
- 任务执行完成后记录耗时
- 使用彩色日志输出提升可读性

### 类型转换

实现了从 `Task` 到 `tokio_cron_scheduler::Job` 的转换，自动包装任务执行逻辑。

## 技术细节

### 任务执行流程

1. 从 cron 表达式解析调度时间
2. 在指定时间触发任务执行
3. 记录任务开始执行日志
4. 执行任务逻辑
5. 记录任务完成日志和执行耗时

### 日志格式

- 任务开始：`[task:任务名称] 开始执行`
- 任务完成：`[task:任务名称] 执行完成，耗时: Xms`

### 时区支持

所有任务调度均使用 `Asia/Shanghai` 时区，确保时间准确性。

## 使用示例

### 定义任务

首先实现 `TaskBuilder` trait：

```rust
use async_trait::async_trait;
use puniyu_builder::task::TaskBuilder;

struct MyTask;

#[async_trait]
impl TaskBuilder for MyTask {
    fn name(&self) -> &'static str {
        "my_task"
    }

    fn cron(&self) -> &'static str {
        "0 0 * * * *"  // 每小时执行一次
    }

    async fn run(&self) {
        println!("执行定时任务...");
        // 在这里实现任务逻辑
    }
}
```

### 初始化调度器并添加任务

```rust
use puniyu_task::{init_scheduler, Task, SCHEDULER};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // 初始化全局调度器
    init_scheduler().await;
    
    // 创建任务
    let task = Task {
        plugin_name: "my_plugin",
        builder: Arc::new(MyTask),
    };
    
    // 将任务转换为 Job 并添加到调度器
    let job = tokio_cron_scheduler::Job::from(task);
    
    if let Some(scheduler) = SCHEDULER.get() {
        scheduler.add(job).await.unwrap();
        scheduler.start().await.unwrap();
    }
    
    // 保持程序运行
    tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
}
```

### Cron 表达式示例

```rust
// 每分钟执行
"0 * * * * *"

// 每小时的第 30 分钟执行
"0 30 * * * *"

// 每天凌晨 2 点执行
"0 0 2 * * *"

// 每周一上午 9 点执行
"0 0 9 * * MON"

// 每个工作日上午 10 点执行
"0 0 10 * * MON-FRI"
```

### 任务执行日志示例

当任务执行时，会自动输出彩色日志：

```
[task:my_task] 开始执行
[task:my_task] 执行完成，耗时: 125ms
```

### 多任务管理

```rust
use puniyu_task::{init_scheduler, Task, SCHEDULER};
use std::sync::Arc;

async fn setup_tasks() {
    init_scheduler().await;
    
    let tasks = vec![
        Task {
            plugin_name: "plugin1",
            builder: Arc::new(Task1),
        },
        Task {
            plugin_name: "plugin2",
            builder: Arc::new(Task2),
        },
    ];
    
    if let Some(scheduler) = SCHEDULER.get() {
        for task in tasks {
            let job = tokio_cron_scheduler::Job::from(task);
            scheduler.add(job).await.unwrap();
        }
        scheduler.start().await.unwrap();
    }
}
```

## 依赖集成

重新导出了以下 crate：

- `tokio_cron_scheduler`: 核心调度器，可直接使用调度器的所有功能
- `uuid::Uuid`: UUID 生成器，用于任务标识

## 技术特性

### 线程安全

- 使用 `Arc` 确保任务构建器的线程安全共享
- 使用 `OnceCell` 确保全局调度器的线程安全初始化（单例模式）
- 基于 tokio 异步运行时实现并发安全
- 所有任务构建器必须实现 `Send + Sync + 'static`

### 性能监控

- 自动记录每个任务的执行时间
- 使用 `Instant` 精确测量任务耗时
- 日志输出包含毫秒级性能指标

### 错误处理

- 调度器初始化使用 `unwrap()`，在生产环境建议增加错误处理
- cron 表达式解析失败会导致任务创建失败
- 建议在实现 `TaskBuilder::run()` 时添加错误处理逻辑

## 依赖

- `tokio`: 异步运行时
- `tokio-cron-scheduler`: cron 任务调度
- `chrono-tz`: 时区支持
- `uuid`: UUID 生成
- `puniyu_logger`: 日志记录
- `puniyu_builder`: 任务构建器接口

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。
# puniyu_task

轻量定时任务接口，基于 Cron 表达式定义任务执行时间。

## 特性

- 提供 `Task` trait 统一定义任务
- 支持 6 位标准 Cron：`秒 分 时 日 月 周`
- 提供实例级 `TaskRegistry` 管理任务注册与调度
- 支持启动前登记任务，以及停止后重新创建调度器

## 快速开始

```rust
use async_trait::async_trait;
use puniyu_error::AnyError;
use puniyu_task::{Task, TaskRegistry};

struct MyTask;

#[async_trait]
impl Task for MyTask {
    fn name(&self) -> &'static str { "my_task" }
    fn cron(&self) -> &'static str { "0 0 3 * * *" }
    async fn execute(&self) -> AnyError { Ok(()) }
}

let registry = TaskRegistry::new();
registry.register(1, MyTask).await?;
registry.start().await?;
registry.stop().await?;
```

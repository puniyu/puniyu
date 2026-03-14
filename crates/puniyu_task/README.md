# puniyu_task

Puniyu 框架的定时任务管理模块，提供基于 Cron 表达式的任务调度功能。

## 功能特性

- ⏰ **Cron 调度**: 使用标准 Cron 表达式定义任务执行时间
- 🚀 **异步执行**: 基于 Tokio 的异步任务执行
- 📝 **任务注册**: 支持动态注册和卸载任务
- 🔗 **插件关联**: 任务与插件 ID 关联，便于管理
- 🔍 **任务查询**: 支持通过 ID、名称或插件 ID 查询任务
- 📊 **执行日志**: 自动记录任务执行状态和耗时
- 🌏 **时区支持**: 默认使用 Asia/Shanghai 时区

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_task = "*"

# 如果需要使用任务注册表功能
puniyu_task = { VERSION = "*", features = ["registry"] }
```

## 核心概念

### Task Trait

所有定时任务都需要实现 `Task` trait：

```rust
use puniyu_task::Task;
use async_trait::async_trait;
use puniyu_error::Result;

struct MyTask;

#[async_trait]
impl Task for MyTask {
    fn name(&self) -> &'static str {
        "my_task"
    }

    fn cron(&self) -> &'static str {
        // 每分钟执行一次
        "0 * * * * *"
    }

    async fn run(&self) -> Result {
        println!("任务执行中...");
        Ok(())
    }
}
```

### TaskRegistry

使用 `TaskRegistry` 管理任务的生命周期：

| 方法                        | 说明                       |
| --------------------------- | -------------------------- |
| `register`                  | 注册新任务                 |
| `unregister`                | 卸载任务（通过 ID 或名称） |
| `unregister_with_plugin_id` | 卸载插件的所有任务         |
| `get`                       | 查询任务信息               |
| `get_with_plugin_id`        | 获取插件的所有任务         |
| `all`                       | 获取所有任务               |

## 使用示例

### 基础使用

```rust
use puniyu_task::{Task, registry::TaskRegistry};
use async_trait::async_trait;
use croner::Cron;
use std::sync::Arc;

// 定义任务
struct DailyTask;

#[async_trait]
impl Task for DailyTask {
    fn name(&self) -> &'static str {
        "daily_task"
    }

    fn cron(&self) -> Cron {
        // 每天凌晨 2 点执行
        Cron::new("0 0 2 * * *").parse().unwrap()
    }

    async fn run(&self) -> puniyu_error::Result {
        // 执行任务逻辑
        println!("执行每日任务");
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let plugin_id = 1;
    let task = Arc::new(DailyTask);

    // 注册任务
    let task_id = TaskRegistry::register(plugin_id, task).await.unwrap();
    println!("任务已注册，ID: {}", task_id);
}
```

### 查询任务

```rust
use puniyu_task::registry::TaskRegistry;

// 通过 ID 查询
let tasks = TaskRegistry::get(task_id);

// 通过名称查询
let tasks = TaskRegistry::get("daily_task");

// 获取插件的所有任务
let tasks = TaskRegistry::get_with_plugin_id(plugin_id);

// 获取所有任务
let all_tasks = TaskRegistry::all();
```

### 卸载任务

```rust
use puniyu_task::TaskRegistry;

// 通过 ID 卸载
TaskRegistry::unregister(task_id).await.unwrap();

// 通过名称卸载
TaskRegistry::unregister("daily_task").await.unwrap();

// 卸载插件的所有任务
TaskRegistry::unregister_with_plugin_id(plugin_id).await.unwrap();
```

### 复杂任务示例

```rust
use puniyu_task::Task;
use async_trait::async_trait;
use puniyu_error::Result;

struct DataBackupTask {
    backup_path: String,
}

#[async_trait]
impl Task for DataBackupTask {
    fn name(&self) -> &'static str {
        "data_backup"
    }

    fn cron(&self) -> &'static str {
        // 每天凌晨 3 点执行
        "0 0 3 * * *"
    }

    async fn run(&self) -> Result {
        println!("开始备份数据到: {}", self.backup_path);

        // 执行备份逻辑
        // ...

        println!("数据备份完成");
        Ok(())
    }
}

// 使用
let task = Arc::new(DataBackupTask {
    backup_path: "/backup/data".to_string(),
});
TaskRegistry::register(1, task).await.unwrap();
```

## Cron 表达式格式

使用标准的 6 位 Cron 表达式：

```text
秒 分 时 日 月 周
*  *  *  *  *  *
```

### 常用示例

| 表达式               | 说明                 |
| -------------------- | -------------------- |
| `0 * * * * *`        | 每分钟执行           |
| `0 0 * * * *`        | 每小时执行           |
| `0 0 0 * * *`        | 每天午夜执行         |
| `0 0 2 * * *`        | 每天凌晨 2 点执行    |
| `0 0 12 * * MON-FRI` | 工作日中午 12 点执行 |
| `0 0 0 1 * *`        | 每月 1 号午夜执行    |
| `0 0 0 * * SUN`      | 每周日午夜执行       |
| `0 */5 * * * *`      | 每 5 分钟执行        |
| `0 0 */2 * * *`      | 每 2 小时执行        |

### 特殊字符

- `*` - 任意值
- `,` - 值列表分隔符（如 `MON,WED,FRI`）
- `-` - 值范围（如 `1-5`）
- `/` - 步长值（如 `*/5` 表示每 5 个单位）

## 任务执行日志

启用 `registry` feature 后，任务执行会自动记录日志：

```text
[task:daily_task] 开始执行
[task:daily_task] 执行完成,耗时: 1234ms
```

如果任务执行失败：

```text
[task:daily_task] 开始执行
[task:daily_task] 执行失败,耗时: 567ms, 错误: Task execution failed
```

## Feature Flags

| Feature    | 说明               | 默认 |
| ---------- | ------------------ | ---- |
| `registry` | 启用任务注册表功能 | ❌   |

启用 `registry` feature 会包含：

- 任务调度器（`tokio-cron-scheduler`）
- 任务存储和管理
- 执行日志记录
- UUID 支持
- 时区支持

## 依赖

- `async-trait` - 异步 trait 支持
- `croner` - Cron 表达式解析
- `tokio-cron-scheduler` - 任务调度器（registry feature）
- `puniyu_error` - 错误处理
- `puniyu_logger` - 日志记录（registry feature）

## 注意事项

1. **时区设置**: 默认使用 `Asia/Shanghai` 时区，任务执行时间基于此时区
2. **任务唯一性**: 同一插件下不能注册同名任务
3. **异步执行**: 所有任务都在异步上下文中执行
4. **错误处理**: 任务执行失败不会影响调度器继续运行
5. **资源清理**: 卸载插件时应同时卸载其所有任务

## 最佳实践

### 1. 任务命名

使用清晰的任务名称：

```rust
fn name(&self) -> &'static str {
    "user_data_cleanup"  // 好
    // "task1"           // 不好
}
```

### 2. 错误处理

在任务中妥善处理错误：

```rust
async fn run(&self) -> Result {
    match self.do_work().await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("任务执行失败: {}", e);
            Err(e)
        }
    }
}
```

### 3. 避免长时间运行

任务应该快速完成，避免阻塞调度器：

```rust
async fn run(&self) -> Result {
    // 好：快速完成
    self.quick_operation().await?;

    // 不好：长时间运行
    // self.long_running_operation().await?;

    Ok(())
}
```

### 4. 资源管理

确保在插件卸载时清理任务：

```rust
// 插件卸载时
TaskRegistry::unregister_with_plugin_id(plugin_id).await?;
```

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_task)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

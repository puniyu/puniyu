# puniyu_task

轻量定时任务接口，基于 Cron 表达式定义任务执行时间。

## 特性

- ⏰ **Cron 调度**: 使用标准 Cron 表达式定义任务执行时间
- 🚀 **异步执行**: 基于 Tokio 的异步任务执行
- 📝 **任务管理**: 支持注册、查询与卸载任务
- 🔗 **插件集成**: 关联插件 ID，支持执行日志与时区调度

## 示例

```rust
use async_trait::async_trait;
use puniyu_task::Task;

struct CleanupTask;

#[async_trait]
impl Task for CleanupTask {
    fn name(&self) -> &'static str { "cleanup" }
    fn cron(&self) -> &'static str { "0 0 3 * * *" }
    async fn run(&self) -> puniyu_error::Result { Ok(()) }
}
```

说明：启用 `registry` feature 后可使用 `TaskRegistry` 进行注册、查询和卸载。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

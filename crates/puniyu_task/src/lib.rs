//! # puniyu_task
//!
//! Puniyu 框架的定时任务管理模块，提供基于 Cron 表达式的任务调度功能。
//!
//! ## 功能特性
//!
//! - **Cron 调度**: 使用标准 Cron 表达式定义任务执行时间
//! - **异步执行**: 基于 Tokio 的异步任务执行
//! - **任务注册**: 支持动态注册和卸载任务
//! - **插件关联**: 任务与插件 ID 关联，便于管理
//! - **任务查询**: 支持通过 ID、名称或插件 ID 查询任务
//! - **执行日志**: 自动记录任务执行状态和耗时
//!
//! ## 核心概念
//!
//! ### Task Trait
//!
//! 所有定时任务都需要实现 [`Task`] trait：
//!
//! ```rust
//! use puniyu_task::Task;
//! use async_trait::async_trait;
//! use puniyu_error::Result;
//!
//! struct MyTask;
//!
//! #[async_trait]
//! impl Task for MyTask {
//!     fn name(&self) -> &'static str {
//!         "my_task"
//!     }
//!
//!     fn cron(&self) -> &'static str {
//!         // 每分钟执行一次
//!         "0 * * * * *"
//!     }
//!
//!     async fn run(&self) -> Result {
//!         println!("任务执行中...");
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ### TaskRegistry
//!
//! 使用 [`TaskRegistry`] 管理任务的生命周期：
//!
//! - `register`: 注册新任务
//! - `unregister`: 卸载任务
//! - `get`: 查询任务信息
//! - `all`: 获取所有任务
//!
//! ## 使用示例
//!
//! ```rust, ignore
//! use puniyu_task::{Task, TaskRegistry};
//! use async_trait::async_trait;
//! use std::sync::Arc;
//!
//! // 定义任务
//! struct DailyTask;
//!
//! #[async_trait]
//! impl Task for DailyTask {
//!     fn name(&self) -> &'static str {
//!         "daily_task"
//!     }
//!
//!     fn cron(&self) -> &'static str {
//!         // 每天凌晨 2 点执行
//!         "0 0 2 * * *"
//!     }
//!
//!     async fn run(&self) -> puniyu_error::Result {
//!         // 执行任务逻辑
//!         println!("执行每日任务");
//!         Ok(())
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let plugin_id = 1;
//!     let task = Arc::new(DailyTask);
//!     
//!     // 注册任务
//!     let task_id = TaskRegistry::register(plugin_id, task).await.unwrap();
//!     println!("任务已注册，ID: {}", task_id);
//!     
//!     // 查询任务
//!     let tasks = TaskRegistry::get(task_id);
//!     println!("找到 {} 个任务", tasks.len());
//!     
//!     // 卸载任务
//!     TaskRegistry::unregister(task_id).await.unwrap();
//!     println!("任务已卸载");
//! }
//! ```
//!
//! ## Cron 表达式格式
//!
//! 使用标准的 6 位 Cron 表达式：
//!
//! ```text
//! 秒 分 时 日 月 周
//! *  *  *  *  *  *
//! ```
//!
//! 示例：
//! - `0 * * * * *` - 每分钟执行
//! - `0 0 * * * *` - 每小时执行
//! - `0 0 0 * * *` - 每天午夜执行
//! - `0 0 12 * * MON-FRI` - 工作日中午 12 点执行
//!
//! ## Feature Flags
//!
//! - `registry`: 启用任务注册表功能（包含调度器和日志）

#[cfg(feature = "registry")]
mod registry;
#[cfg(feature = "registry")]
pub use registry::{TaskRegistry, init};
mod types;
#[doc(inline)]
pub use types::*;

use async_trait::async_trait;
use puniyu_error::Result;

/// 定时任务 trait
///
/// 所有定时任务都需要实现此 trait，定义任务的名称、执行时间和执行逻辑。
///
/// # 必需方法
///
/// - `name()` - 返回任务的唯一名称
/// - `cron()` - 返回 Cron 表达式字符串
/// - `run()` - 异步执行任务逻辑
///
/// # Cron 表达式格式
///
/// 使用标准的 6 位 Cron 表达式：`秒 分 时 日 月 周`
///
/// 示例：
/// - `"0 * * * * *"` - 每分钟执行
/// - `"0 0 * * * *"` - 每小时执行
/// - `"0 0 0 * * *"` - 每天午夜执行
/// - `"0 0 12 * * MON-FRI"` - 工作日中午 12 点执行
///
/// # 示例
///
/// ```rust
/// use puniyu_task::Task;
/// use async_trait::async_trait;
/// use puniyu_error::Result;
///
/// struct MyTask;
///
/// #[async_trait]
/// impl Task for MyTask {
///     fn name(&self) -> &'static str {
///         "my_task"
///     }
///
///     fn cron(&self) -> &'static str {
///         // 每分钟执行一次
///         "0 * * * * *"
///     }
///
///     async fn run(&self) -> Result {
///         println!("任务执行中...");
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait Task: Send + Sync + 'static {
	/// 获取任务名称
	///
	/// # 返回值
	///
	/// 返回任务的唯一标识名称。
	fn name(&self) -> &'static str;

	/// 获取 Cron 表达式
	///
	/// # 返回值
	///
	/// 返回定义任务执行时间的 Cron 表达式字符串。
	///
	/// # Cron 格式
	///
	/// 使用 6 位格式：`秒 分 时 日 月 周`
	fn cron(&self) -> &'static str;

	/// 执行任务
	///
	/// # 返回值
	///
	/// 返回任务执行结果，成功返回 `Ok(())`，失败返回错误信息。
	///
	/// # 错误
	///
	/// 如果任务执行过程中发生错误，应返回相应的错误信息。
	async fn run(&self) -> Result;
}


impl PartialEq for dyn Task {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name() && self.cron() == other.cron()
	}
}
//! # puniyu_task
//!
//! 轻量定时任务接口，基于 Cron 表达式定义任务执行时间。
//!
//! ## 特性
//!
//! - `Task` trait：统一定义任务名称、Cron 和执行逻辑
//! - 标准 6 位 Cron：`秒 分 时 日 月 周`
//! - `registry` feature：启用任务注册与调度能力
//!
//! ## 示例
//!
//! ```rust
//! use puniyu_task::Task;
//! use async_trait::async_trait;
//! use puniyu_error::AnyError;
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
//!         "0 0 3 * * *"
//!     }
//!
//!     async fn execute(&self) -> AnyError {
//!         Ok(())
//!     }
//! }
//! ```
//!
//! 说明：启用 `registry` feature 后可使用 [`TaskRegistry`] 进行注册、查询和卸载。

mod error;
pub use error::Error;

mod registry;
pub use registry::TaskRegistry;

mod types;
#[doc(inline)]
pub use types::*;

use async_trait::async_trait;
use puniyu_error::AnyError;
use std::sync::Arc;
use tokio::sync::{Mutex, SetOnce};
use tokio_cron_scheduler::JobScheduler;

static SCHEDULER: SetOnce<Arc<Mutex<JobScheduler>>> = SetOnce::const_new();

/// 启动任务调度器
///
/// 多次调用不会重复初始化，直接返回已初始化的调度器。
pub async fn start_scheduler() -> Result<(), Error> {
	if SCHEDULER.get().is_some() {
		return Ok(());
	}

	let sched = JobScheduler::new().await.map_err(|e| Error::Create(e.to_string()))?;
	sched.start().await.map_err(|e| Error::Start(e.to_string()))?;
	let arc = Arc::new(Mutex::new(sched));

	SCHEDULER.set(arc).map_err(|_| Error::AlreadyInitialized)?;
	Ok(())
}

/// 获取调度器
pub fn get_scheduler() -> Result<Arc<Mutex<JobScheduler>>, Error> {
	SCHEDULER.get().cloned().ok_or(Error::NotInitialized)
}

/// 停止任务调度器
pub async fn stop_scheduler() -> Result<(), Error> {
	let arc = get_scheduler()?;
	let mut sched = arc.lock().await;
	sched.shutdown().await.map_err(|e| Error::Shutdown(e.to_string()))?;
	Ok(())
}

/// 定时任务 trait。
///
/// 实现此 trait 来定义任务名称、Cron 表达式和异步执行逻辑。
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
/// use puniyu_error::AnyError;
///
/// struct MyTask;
///
/// #[async_trait]
/// impl Task for MyTask {
///     fn name(&self) -> &'static str {
///         "my_task"
///     }
///
///     fn cron(&self) -> &'static str { "0 * * * * *" }
///
///     async fn execute(&self) -> AnyError {
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait Task: Send + Sync {
	/// 获取任务名称
	fn name(&self) -> &str;

	/// 获取 Cron 表达式
	///
	/// # Cron 格式
	///
	/// 使用 6 位格式：`秒 分 时 日 月 周`
	fn cron(&self) -> &str;

	/// 执行任务
	async fn execute(&self) -> AnyError;
}

impl PartialEq for dyn Task {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name() && self.cron() == other.cron()
	}
}

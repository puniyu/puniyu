//! # puniyu_task
//!
//! 轻量定时任务接口，基于 Cron 表达式定义任务执行时间。
//!
//! ## 特性
//!
//! - `Task` trait：统一定义任务名称、Cron 和执行逻辑
//! - 标准 6 位 Cron：`秒 分 时 日 月 周`
//! - [`TaskRegistry`]：基于无锁并发的注册表，支持调度器生命周期管理
//! - 使用 `tokio-cron-scheduler` 驱动定时调度
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

mod error;
pub use error::Error;

mod registry;
pub use registry::TaskRegistry;

mod scheduler;

mod types;
#[doc(inline)]
pub use types::*;

use async_trait::async_trait;
use puniyu_error::AnyError;

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

#[async_trait]
impl<T: Task + ?Sized> Task for Box<T> {
	fn name(&self) -> &str {
		self.as_ref().name()
	}

	fn cron(&self) -> &str {
		self.as_ref().cron()
	}

	async fn execute(&self) -> AnyError {
		self.as_ref().execute().await
	}
}

#[async_trait]
impl<T: Task + ?Sized> Task for std::sync::Arc<T> {
	fn name(&self) -> &str {
		self.as_ref().name()
	}

	fn cron(&self) -> &str {
		self.as_ref().cron()
	}

	async fn execute(&self) -> AnyError {
		self.as_ref().execute().await
	}
}

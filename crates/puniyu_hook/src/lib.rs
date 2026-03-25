//! # puniyu_hook
//!
//! 钩子系统库，提供事件钩子的定义和管理。
//!
//! ## 概述
//!
//! `puniyu_hook` 提供了钩子系统的核心功能，允许在特定事件发生时执行自定义逻辑。
//! 钩子可以用于日志记录、权限检查、数据处理等场景。
//!
//! ## 使用方式
//!
//! ### 实现钩子
//!
//! ```rust,ignore
//! use puniyu_hook::{Hook, types::HookType};
//! use puniyu_context::EventContext;
//! use async_trait::async_trait;
//!
//! struct MyHook;
//!
//! #[async_trait]
//! impl Hook for MyHook {
//!     fn name(&self) -> &'static str {
//!         "my_hook"
//!     }
//!
//!     fn r#type(&self) -> &HookType {
//!         &HookType::Event(HookEventType::Message)
//!     }
//!
//!     fn rank(&self) -> u32 {
//!         100
//!     }
//!
//!     async fn run(&self, ctx: Option<&EventContext>) -> puniyu_error::Result {
//!         if let Some(ctx) = ctx {
//!             println!("处理事件: {:?}", ctx.event());
//!         }
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ### 使用钩子
//!
//! ```rust,ignore
//! use puniyu_hook::Hook;
//!
//! async fn execute_hook(hook: &dyn Hook, ctx: &EventContext) {
//!     match hook.run(Some(ctx)).await {
//!         Ok(_) => println!("钩子执行成功"),
//!         Err(e) => eprintln!("钩子执行失败: {}", e),
//!     }
//! }
//! ```

#[cfg(feature = "registry")]
mod registry;
use async_trait::async_trait;
use puniyu_context::EventContext;
#[cfg(feature = "registry")]
#[doc(inline)]
pub use registry::HookRegistry;
mod types;
#[doc(inline)]
pub use types::*;
#[doc(inline)]
pub use puniyu_common::source::SourceType;

/// 钩子 Trait
///
/// 定义了钩子的基本接口。所有钩子实现都必须实现此 trait。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_hook::{Hook, types::HookType};
/// use puniyu_context::EventContext;
/// use async_trait::async_trait;
///
/// struct LogHook;
///
/// #[async_trait]
/// impl Hook for LogHook {
///     fn name(&self) -> &'static str {
///         "log_hook"
///     }
///
///     fn r#type(&self) -> &HookType {
///         &HookType::Event(HookEventType::Message)
///     }
///
///     fn rank(&self) -> u32 {
///         100
///     }
///
///     async fn run(&self, ctx: Option<&EventContext>) -> puniyu_error::Result {
///         println!("钩子被触发");
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait Hook: Send + Sync + 'static {
	/// 获取钩子名称
	///
	/// 返回钩子的唯一标识名称。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// fn name(&self) -> &'static str {
	///     "my_hook"
	/// }
	/// ```
	fn name(&self) -> &'static str;

	/// 获取钩子类型
	///
	/// 返回钩子的类型，用于确定钩子在何时触发。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// fn r#type(&self) -> &HookType {
	///     &HookType::Event(HookEventType::Message)
	/// }
	/// ```
	fn r#type(&self) -> &HookType;

	/// 获取钩子优先级
	///
	/// 返回钩子的执行优先级，数值越小优先级越高。
	///
	/// # 返回值
	///
	/// 返回优先级数值，默认为 100
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// fn rank(&self) -> u32 {
	///     50  // 高优先级
	/// }
	/// ```
	fn rank(&self) -> u32;

	/// 执行钩子逻辑
	///
	/// 当钩子被触发时调用此方法。
	///
	/// # 参数
	///
	/// - `ctx` - 可选的事件上下文
	///
	/// # 错误
	///
	/// 如果钩子执行失败，返回错误
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// async fn run(&self, ctx: Option<&EventContext>) -> puniyu_error::Result {
	///     if let Some(ctx) = ctx {
	///         // 处理事件
	///         println!("处理事件: {:?}", ctx.event());
	///     }
	///     Ok(())
	/// }
	/// ```
	async fn run(&self, ctx: Option<&EventContext>) -> puniyu_error::Result;
}

//! # puniyu_hook
//!
//! 钩子系统库，提供统一的 `Hook` 接口和钩子类型定义。
//!
//! ## 特性
//!
//! - `Hook` trait 定义钩子执行模型
//! - 提供事件/状态钩子类型
//! - 可选 `registry` 功能用于钩子注册管理

#[cfg(feature = "registry")]
mod registry;
use async_trait::async_trait;
use puniyu_context::EventContext;
#[cfg(feature = "registry")]
#[doc(inline)]
pub use registry::HookRegistry;
mod types;
#[doc(inline)]
pub use puniyu_common::source::SourceType;
#[doc(inline)]
pub use types::*;

/// 钩子接口。
#[async_trait]
pub trait Hook: Send + Sync + 'static {
	/// 获取钩子名称。
	fn name(&self) -> &'static str;

	/// 获取钩子类型。
	fn r#type(&self) -> &HookType;

	/// 获取钩子优先级（值越小优先级越高）。
	fn priority(&self) -> u32;

	/// 执行钩子逻辑。
	async fn execute(&self, ctx: Option<&EventContext>) -> puniyu_error::Result;
}

impl PartialEq for dyn Hook {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name()
			&& self.r#type() == other.r#type()
			&& self.priority() == other.priority()
	}
}

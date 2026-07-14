//! # puniyu_handler
//!
//! 事件处理器库，提供统一的 `Handler` 接口。
//!
//! ## 特性
//!
//! - `Handler` trait 定义事件处理模型
//! - 支持处理 `puniyu_event::Event`
//! - 支持前置、后置和短路的洋葱调用链
//! - 提供处理器注册管理

mod error;
pub use error::Error;

mod types;
#[doc(inline)]
pub use types::*;
mod registry;
#[doc(inline)]
pub use registry::HandlerRegistry;

use async_trait::async_trait;
/// 事件处理器接口。
#[async_trait]
pub trait Handler: Send + Sync + 'static {
	/// 获取处理器名称。
	fn name(&self) -> &'static str;

	/// 处理事件。
	///
	/// 调用 [`HandleContext::next`] 进入后续处理器；不调用则正常终止调用链。
	async fn handle(&self, ctx: HandleContext<'_, '_>);
}

impl PartialEq for dyn Handler {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name()
	}
}

impl Eq for dyn Handler {}

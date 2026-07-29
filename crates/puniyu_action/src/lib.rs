//! # puniyu_action
//!
//! 统一的 puniyu 行为库，提供 [`Action`] trait 定义行为。
//!
//! 每个 Action 必然包含：
//! - **Matcher**：负责匹配事件
//! - **Handler**：负责执行行为逻辑
//!
//! 可选包含：
//! - **Extractor**：负责从事件中提取数据
//!
//! 执行流程由调度层内部处理：matcher.matches → handler.handle

mod types;
#[doc(inline)]
pub use types::*;
mod registry;
pub use registry::ActionRegistry;

use std::sync::Arc;

use puniyu_handler::Handler;
use puniyu_matcher::Matcher;


pub trait Action: Send + Sync {
	/// 行为名称
	fn name(&self) -> &str;

	/// 行为匹配器
	fn matcher(&self) -> &dyn Matcher;

	/// 行为处理器
	fn handler(&self) -> &dyn Handler;

	/// 行为描述
	fn description(&self) -> Option<&str> {
		None
	}

	/// 优先级
	fn priority(&self) -> u32 {
		500
	}

	/// 匹配成功后是否阻断后续行为。默认 `false`。
	fn block(&self) -> bool {
		false
	}
}


impl<A: Action + ?Sized> Action for Arc<A> {
	fn name(&self) -> &str {
		(**self).name()
	}

	fn matcher(&self) -> &dyn Matcher {
		(**self).matcher()
	}

	fn handler(&self) -> &dyn Handler {
		(**self).handler()
	}

	fn description(&self) -> Option<&str> {
		(**self).description()
	}

	fn priority(&self) -> u32 {
		(**self).priority()
	}

	fn block(&self) -> bool {
		(**self).block()
	}
}

impl<A: Action + ?Sized> Action for Box<A> {
	fn name(&self) -> &str {
		(**self).name()
	}

	fn matcher(&self) -> &dyn Matcher {
		(**self).matcher()
	}

	fn handler(&self) -> &dyn Handler {
		(**self).handler()
	}

	fn description(&self) -> Option<&str> {
		(**self).description()
	}

	fn priority(&self) -> u32 {
		(**self).priority()
	}

	fn block(&self) -> bool {
		(**self).block()
	}
}

impl<A: Action + ?Sized> Action for &A {
	fn name(&self) -> &str {
		(**self).name()
	}

	fn matcher(&self) -> &dyn Matcher {
		(**self).matcher()
	}

	fn handler(&self) -> &dyn Handler {
		(**self).handler()
	}

	fn description(&self) -> Option<&str> {
		(**self).description()
	}

	fn priority(&self) -> u32 {
		(**self).priority()
	}

	fn block(&self) -> bool {
		(**self).block()
	}
}

//! # puniyu_command
//!
//! 统一的 puniyu 命令库，提供 [`Command`] trait 定义命令行为。
//!
//! 命令由 [`Matcher`](puniyu_matcher::Matcher) 和 [`CommandHandler`](puniyu_handler::CommandHandler) 组合而成：
//! - **Matcher**：负责匹配消息、解析参数、检查权限
//! - **Handler**：负责执行命令逻辑

mod types;
#[doc(inline)]
pub use types::*;

pub type CommandRegistry = puniyu_registry::Registry<std::sync::Arc<dyn Command>>;

use async_trait::async_trait;

#[async_trait]
pub trait Command: Send + Sync {
	/// 命令名称（用于日志/帮助）。
	fn name(&self) -> &str;

	/// 优先级
	fn priority(&self) -> u32 {
		500
	}

	/// 命令描述。
	fn description(&self) -> Option<&str> {
		None
	}

	/// 匹配成功后是否阻断后续命令。默认 `false`。
	fn block(&self) -> bool {
		false
	}

	/// 返回此命令的处理器。
	async fn execute(
		&self, 
		session: &puniyu_session::MessageSession
	) -> puniyu_error::AnyError;
}

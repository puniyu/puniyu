mod arg;
pub use arg::*;

use crate::context::{BotContext, MessageContext};
use crate::event::Permission;
use async_trait::async_trait;

/// 命令处理动作
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerAction {
	/// 处理完成，停止传播
	Done,
	/// 继续传播给其他处理器
	Continue,
}

impl HandlerAction {
	pub const fn done() -> HandlerResult {
		Ok(HandlerAction::Done)
	}
	pub const fn r#continue() -> HandlerResult {
		Ok(HandlerAction::Continue)
	}
}

impl From<()> for HandlerAction {
	fn from(_: ()) -> Self {
		HandlerAction::Done
	}
}

/// 命令处理结果
pub type HandlerResult = Result<HandlerAction, Box<dyn std::error::Error + Send + Sync>>;


/// 命令
#[derive(Debug, Clone)]
pub struct Command {
	pub name: &'static str,
	pub description: Option<&'static str>,
	pub args: Vec<Arg>,
	pub rank: u64,
	/// 自定义前缀，None 表示使用全局前缀
	pub prefix: Option<String>,
	/// 命令别名
	pub alias: Option<Vec<&'static str>>,
	/// 权限等级
	pub permission: Permission,
}

#[async_trait]
pub trait CommandBuilder: Send + Sync + 'static {
	/// 命令名称
	fn name(&self) -> &'static str;

	/// 描述
	fn description(&self) -> Option<&'static str>;

	/// 参数列表
	fn args(&self) -> Vec<Arg>;

	/// 优先级
	fn rank(&self) -> u64;

	/// 命令别名
	fn alias(&self) -> Option<Vec<&'static str>> {
		None
	}

	/// 权限等级
	fn permission(&self) -> Permission {
		Permission::All
	}

	/// 执行的函数
	async fn run(&self, bot: &BotContext, ev: &MessageContext) -> HandlerResult;
}

mod store;
mod registry;
pub use registry::CommandRegistry;

use async_trait::async_trait;
use crate::context::{MessageContext, BotContext};

#[derive(Debug, Clone)]
pub enum HandlerResult {
	/// 处理完成
	Ok,
	/// 继续处理
	Continue,
}

#[async_trait]
pub trait CommandBuilder: Send + Sync + 'static {
	/// 命令
	///
	/// ## 示例
	///
	/// ```ignore
	/// echo
	/// ```
	///
	/// ## 说明
	///
	/// - `echo` 表示命令名称
	fn name(&self) -> &'static str;

	/// 描述
	fn description(&self) -> Option<&'static str>;

	/// 参数
	///
	/// 此命令所拥有的参数列表
	fn args(&self) -> Vec<String>;

	/// 优先级
	fn rank(&self) -> usize;

	/// 执行的函数
	async fn run(&self, bot: &BotContext, ev: &MessageContext) -> HandlerResult;
}
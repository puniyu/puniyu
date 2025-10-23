mod registry;
mod store;
#[cfg(feature = "registry")]
pub use registry::CommandRegistry;

#[cfg(feature = "builder")]
use async_trait::async_trait;
#[cfg(feature = "builder")]
use puniyu_event::context::{BotContext, MessageContext};
#[cfg(feature = "builder")]
use std::sync::Arc;

#[cfg(feature = "builder")]
#[derive(Debug, Clone)]
pub enum HandlerResult {
	/// 处理完成
	Ok,
	/// 继续处理
	Continue,
}

#[cfg(feature = "builder")]
#[derive(Clone)]
pub struct Command {
	/// 插件名称
	pub plugin_name: &'static str,
	/// 命令名称
	pub builder: Arc<dyn CommandBuilder>,
}

#[cfg(feature = "builder")]
#[macro_export]
macro_rules! register_command {
	($plugin_name:expr, $command_builder:expr) => {{
		let command =
			Command { plugin_name: $plugin_name, builder: std::sync::Arc::from($command_builder) };
		CommandRegistry::insert(command);
	}};
}
/// 命令构造器
#[cfg(feature = "builder")]
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
	/// 此命令所拥有的参数
	fn args(&self) -> Vec<String>;

	/// 优先级
	fn rank(&self) -> usize;

	/// 执行的函数
	async fn run(&self, bot: &BotContext, ev: &MessageContext) -> HandlerResult;
}

use crate::{bot::Bot, plugin::command::Command};
use async_trait::async_trait;

#[async_trait]
pub trait CommandBuilder: Send + Sync + 'static {
	/// 命令名称
	fn name(&self) -> &'static str;
	/// 命令
	///
	/// ## 示例
	///
	/// ```ignore
	/// echo <String>
	/// ```
	///
	/// ## 说明
	///
	/// - `<String>` 表示参数，参数类型为 `String`
	/// - `echo` 表示命令名称
	fn command(&self) -> &'static str;

	/// 优先级
	fn rank(&self) -> usize;

	/// TODO: e占位，未实现
	async fn run(&self, bot: &Bot);
}

impl From<Box<dyn CommandBuilder>> for Command {
	fn from(command_builder: Box<dyn CommandBuilder>) -> Self {
		Self {
			name: command_builder.name(),
			command: command_builder.command(),
			rank: command_builder.rank(),
		}
	}
}

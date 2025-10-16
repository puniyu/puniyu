use async_trait::async_trait;
use puniyu_event_context::{Bot, EventContext};

/// 命令构造器
#[async_trait]
pub trait CommandBuilder: Send + Sync + 'static {
	/// 命令名称
	fn name(&self) -> &'static str;
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
	fn command(&self) -> &'static str;

	/// 描述
	fn description(&self) -> Option<&'static str>;

	/// 参数
	///
	/// 此命令所拥有的参数
	fn args(&self) -> Vec<String>;

	/// 优先级
	fn rank(&self) -> usize;

	/// 执行的函数
	async fn run(&self, bot: &Bot, ev: &EventContext);
}

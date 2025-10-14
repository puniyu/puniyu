pub mod builder;
mod registry;

#[derive(Debug, Clone)]
pub struct Command {
	/// 命令名称
	pub name: &'static str,
	/// 命令
	pub command: &'static str,
	/// 命令优先级
	pub rank: usize,
}

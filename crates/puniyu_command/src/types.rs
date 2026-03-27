#[doc(inline)]
pub use puniyu_command_types::*;
use std::sync::Arc;

/// 命令信息
///
/// 包含命令的元信息和构建器。
#[derive(Clone)]
pub struct CommandInfo {
	/// 所属插件 ID。
	pub plugin_id: u64,
	/// 命令构建器。
	pub builder: Arc<dyn crate::Command>,
}

impl PartialEq for CommandInfo {
	fn eq(&self, other: &Self) -> bool {
		self.plugin_id == other.plugin_id && self.builder.name() == other.builder.name()
	}
}

/// 命令标识符。
pub enum CommandId<'c> {
	/// 通过索引标识。
	Id(u64),
	/// 通过名称标识。
	Name(&'c str),
}

impl From<u64> for CommandId<'_> {
	fn from(id: u64) -> Self {
		Self::Id(id)
	}
}

impl<'c> From<&'c str> for CommandId<'c> {
	fn from(name: &'c str) -> Self {
		Self::Name(name)
	}
}

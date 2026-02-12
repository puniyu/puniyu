//! 命令类型定义模块

#[doc(inline)]
pub use puniyu_command_core::{Arg, CommandAction, Permission};
use std::sync::Arc;

/// 命令信息
///
/// 包含命令的元数据和构建器。
///
/// # 字段
///
/// - `plugin_id` - 所属插件的 ID
/// - `builder` - 命令构建器（实现了 `Command` trait）
#[derive(Clone)]
pub struct CommandInfo {
	/// 插件 ID
	pub plugin_id: u64,
	/// 命令构建器
	pub builder: Arc<dyn crate::Command>,
}

impl PartialEq for CommandInfo {
	fn eq(&self, other: &Self) -> bool {
		self.plugin_id == other.plugin_id && self.builder.name() == other.builder.name()
	}
}

/// 命令 ID
///
/// 用于标识命令的枚举类型，支持通过索引或名称来标识。
///
/// # 示例
///
/// ```rust
/// use puniyu_command::CommandId;
///
/// // 通过索引创建
/// let id1: CommandId = 0u64.into();
///
/// // 通过名称创建
/// let id2: CommandId = "hello".into();
/// ```
pub enum CommandId<'c> {
	/// 通过索引标识
	Id(u64),
	/// 通过名称标识
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

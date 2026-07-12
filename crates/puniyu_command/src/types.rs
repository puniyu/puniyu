#[doc(inline)]
pub use puniyu_command_types::*;
use std::borrow::Cow;

/// 命令标识符。
pub enum CommandId<'c> {
	/// 通过索引标识。
	Id(u64),
	/// 通过名称标识。
	Name(Cow<'c, str>),
}

impl From<u64> for CommandId<'_> {
	fn from(id: u64) -> Self {
		Self::Id(id)
	}
}

impl<'c> From<&'c str> for CommandId<'c> {
	fn from(name: &'c str) -> Self {
		Self::Name(Cow::Borrowed(name))
	}
}

impl From<String> for CommandId<'_> {
	fn from(name: String) -> Self {
		Self::Name(Cow::Owned(name))
	}
}

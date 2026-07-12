use puniyu_error::AnyError;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 命令执行动作。
#[derive(
	Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, EnumString, Display, IntoStaticStr,
)]
pub enum CommandAction {
	/// 处理完成，停止传播。
	Done,
	/// 继续向下传播。
	Next,
}

impl CommandAction {
	/// 返回 `Done` 动作结果。
	pub const fn done() -> AnyError<CommandAction> {
		Ok(Self::Done)
	}

	/// 返回 `Next` 动作结果。
	pub const fn next() -> AnyError<CommandAction> {
		Ok(Self::Next)
	}
}

impl From<()> for CommandAction {
	fn from(_: ()) -> Self {
		Self::Done
	}
}

impl From<bool> for CommandAction {
	fn from(value: bool) -> Self {
		match value {
			true => Self::Done,
			_ => Self::Next,
		}
	}
}

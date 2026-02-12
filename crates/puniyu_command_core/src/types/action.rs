//! 命令执行动作定义

use puniyu_error::Result;

/// 命令处理动作
///
/// 定义命令处理器执行后的行为。
///
/// # 示例
///
/// ```rust
/// use puniyu_command_core::CommandAction;
///
/// // 处理完成，停止传播
/// let done = CommandAction::Done;
///
/// // 继续向下传播
/// let continue_action = CommandAction::Continue;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandAction {
	/// 处理完成，停止传播
	///
	/// 当命令处理器返回此动作时，命令不会继续传播给其他处理器。
	Done,
	/// 继续向下传播给其他处理器
	///
	/// 当命令处理器返回此动作时，命令会继续传播给优先级更低的处理器。
	Continue,
}

impl CommandAction {
	/// 创建 Done 动作的 Result
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_command_core::CommandAction;
	///
	/// let result = CommandAction::done();
	/// assert!(result.is_ok());
	/// ```
	pub const fn done() -> Result<CommandAction> {
		Ok(CommandAction::Done)
	}

	/// 创建 Continue 动作的 Result
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_command_core::CommandAction;
	///
	/// let result = CommandAction::r#continue();
	/// assert!(result.is_ok());
	/// ```
	pub const fn r#continue() -> Result<CommandAction> {
		Ok(CommandAction::Continue)
	}
}

impl From<()> for CommandAction {
	fn from(_: ()) -> Self {
		CommandAction::Done
	}
}

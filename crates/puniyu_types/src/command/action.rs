use crate::handler::HandlerResult;

/// 命令处理动作
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandAction {
    /// 处理完成，停止传播
    Done,
    /// 继续传播给其他处理器
    Continue,
}

impl CommandAction {
    pub const fn done() -> HandlerResult<CommandAction> {
        Ok(CommandAction::Done)
    }
    pub const fn r#continue() -> HandlerResult<CommandAction> {
        Ok(CommandAction::Continue)
    }
}

impl From<()> for CommandAction {
    fn from(_: ()) -> Self {
        CommandAction::Done
    }
}
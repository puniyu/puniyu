use std::sync::Arc;
#[doc(inline)]
pub use puniyu_command_core::{Arg, CommandAction, Permission};

#[derive(Clone)]
pub struct CommandInfo {
    /// 插件id
    pub plugin_id: u64,
    /// 命令构建器
    pub builder: Arc<dyn crate::Command>,
}

impl PartialEq for CommandInfo {
    fn eq(&self, other: &Self) -> bool {
        self.plugin_id == other.plugin_id && self.builder.name() == other.builder.name()
    }
}

pub enum CommandId<'c> {
    /// 命令id
    Id(u64),
    /// 命令名称
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

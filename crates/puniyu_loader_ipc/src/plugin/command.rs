
use crate::CommandMetaData;
use async_trait::async_trait;
use puniyu_command::CommandAction;

pub struct IpcCommand {
    meta: CommandMetaData
}

#[async_trait]
impl puniyu_command::Command for IpcCommand {
    fn name(&self) ->  &str {
        self.meta.name.as_str()
    }
    fn description(&self) -> Option<&str> {
        self.meta.description.as_deref()
    }
    fn args(&self) -> Vec<puniyu_command::Arg> {
        Vec::new()
    }
    fn alias(&self) -> Vec<&str> {
        self.meta.alias.iter().map(|s| s.as_str()).collect()
    }
    fn priority(&self) -> u32 {
        self.meta.priority
    }
    fn permission(&self) -> puniyu_command::Permission {
        self.meta.permission
    }
    async fn execute(&self, _ctx: &puniyu_context::MessageContext) -> puniyu_error::Result<CommandAction> {
       Ok(().into())
    }
}
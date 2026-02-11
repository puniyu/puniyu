mod store;

use std::sync::LazyLock;
use store::CommandStore;
use puniyu_error::registry::Error;
use crate::types::{CommandId, CommandInfo};

static STORE: LazyLock<CommandStore> = LazyLock::new(CommandStore::new);

pub struct CommandRegistry;

impl<'c> CommandRegistry {
    /// 注册命令
    pub fn register<C>(command: C) -> Result<u64, Error>
    where
        C: Into<CommandInfo>,
    {
        let command = command.into();
        STORE.insert(command)
    }
    /// 卸载已注册的指令
    pub fn unregister<C>(command: C) -> Result<(), Error>
    where
        C: Into<CommandId<'c>>,
    {
        let command = command.into();
        match command {
            CommandId::Id(v) => Self::unregister_with_index(v),
            CommandId::Name(v) => Self::unregister_with_command_name(v),
        }
    }

    pub fn unregister_with_index(command_id: u64) -> Result<(), Error> {
        let raw = STORE.raw();
        let mut map = raw.write().expect("Failed to acquire lock");
        if map.get(&command_id).is_none() {
            return Err(Error::NotFound("Command".to_string()));
        }
        map.remove(&command_id);
        Ok(())
    }

    pub fn unregister_with_command_name(name: &str) -> Result<(), Error> {
        let raw = STORE.raw();
        let mut map = raw.write().expect("Failed to acquire lock");
        if !map.values().any(|v| v.builder.name() == name) {
            return Err(Error::NotFound("Command".to_string()));
        }
        map.retain(|_, v| v.builder.name() != name);
        Ok(())
    }

    pub fn unregister_with_plugin_id(plugin_id: u64) -> Result<(), Error> {
        let raw = STORE.raw();
        let mut map = raw.write().expect("Failed to acquire lock");
        if !map.values().any(|v| v.plugin_id == plugin_id) {
            return Err(Error::NotFound("Command".to_string()));
        }
        map.retain(|_, v| v.plugin_id != plugin_id);
        Ok(())
    }
    pub fn get<C, D>(command: C) -> Vec<CommandInfo>
    where
        C: Into<CommandId<'c>>,
    {
        let command = command.into();
        match command {
            CommandId::Id(v) => Self::get_with_command_id(v).into_iter().collect(),
            CommandId::Name(v) => Self::get_with_command_name(v),
        }
    }
    pub fn get_with_command_id(id: u64) -> Option<CommandInfo> {
        let raw = STORE.raw();
        let map = raw.read().expect("Failed to acquire lock");
        map.get(&id).cloned()
    }

    pub fn get_with_command_name(name: &str) -> Vec<CommandInfo> {
        let raw = STORE.raw();
        let map = raw.read().expect("Failed to acquire lock");
        map.values().filter(|v| v.builder.name() == name).cloned().collect::<Vec<CommandInfo>>()
    }

    pub fn get_with_plugin_id(plugin_id: u64) -> Vec<CommandInfo> {
        let raw = STORE.raw();
        let map = raw.read().expect("Failed to acquire lock");
        map.values().filter(|v| v.plugin_id == plugin_id).cloned().collect::<Vec<CommandInfo>>()
    }

    pub fn all() -> Vec<CommandInfo> {
        STORE.all()
    }
}
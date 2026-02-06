use super::types::CommandInfo;
use crate::{Error, Result};
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};

static COMMAND_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
pub(crate) struct CommandStore<'c>(Arc<RwLock<HashMap<u64, CommandInfo<'c>>>>);

impl<'c> CommandStore<'c> {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn insert(&self, command: CommandInfo<'c>) -> Result<u64> {
		let index = COMMAND_ID.fetch_add(1, Ordering::Relaxed);
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v.plugin_id == command.plugin_id && v.name == command.name) {
			return Err(Error::Exists("Command".to_string()));
		}
		map.insert(index, command).ok_or(Error::Exists("Command".to_string()))?;
		Ok(index)
	}
	pub fn all(&self) -> Vec<CommandInfo<'c>> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}
	pub(crate) fn raw(&self) -> Arc<RwLock<HashMap<u64, CommandInfo<'c>>>> {
		self.0.clone()
	}
}

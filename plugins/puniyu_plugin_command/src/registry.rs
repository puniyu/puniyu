use puniyu_command::Command;
use puniyu_context::{PluginContext, ScopeId};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("command id exhausted")]
	IdExhausted,
	#[error("command registry lock is poisoned")]
	Poisoned,
	#[error("command {0} is not owned by the current plugin")]
	NotOwned(u64),
}

struct Entry {
	owner: ScopeId,
	command: Arc<dyn Command>,
}

#[derive(Default)]
struct State {
	next_id: u64,
	commands: BTreeMap<u64, Entry>,
}

#[derive(Clone, Default)]
pub struct CommandRegistry {
	inner: Arc<RwLock<State>>,
}

impl CommandRegistry {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn register(
		&self,
		ctx: &PluginContext,
		command: impl Into<Arc<dyn Command>>,
	) -> Result<u64, Error> {
		let mut state = self.inner.write().map_err(|_| Error::Poisoned)?;
		let id = state.next_id;
		state.next_id = state.next_id.checked_add(1).ok_or(Error::IdExhausted)?;
		state.commands.insert(id, Entry { owner: ctx.scope_id(), command: command.into() });
		Ok(id)
	}

	pub fn unregister(&self, ctx: &PluginContext, id: u64) -> Result<(), Error> {
		let mut state = self.inner.write().map_err(|_| Error::Poisoned)?;
		if state.commands.get(&id).is_none_or(|entry| entry.owner != ctx.scope_id()) {
			return Err(Error::NotOwned(id));
		}
		state.commands.remove(&id);
		Ok(())
	}

	pub fn unregister_scope(&self, ctx: &PluginContext) -> Result<(), Error> {
		self.inner
			.write()
			.map_err(|_| Error::Poisoned)?
			.commands
			.retain(|_, entry| entry.owner != ctx.scope_id());
		Ok(())
	}

	pub fn all(&self) -> Result<Vec<Arc<dyn Command>>, Error> {
		Ok(self
			.inner
			.read()
			.map_err(|_| Error::Poisoned)?
			.commands
			.values()
			.map(|entry| Arc::clone(&entry.command))
			.collect())
	}

	pub(crate) fn clear(&self) -> Result<(), Error> {
		self.inner.write().map_err(|_| Error::Poisoned)?.commands.clear();
		Ok(())
	}
}

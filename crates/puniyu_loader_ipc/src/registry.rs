mod store;

use std::sync::{Arc, LazyLock};

use puniyu_error::registry::Error;
use store::IpcPluginStore;

use crate::types::{IpcPluginId, IpcProcess, ProcessState};

static STORE: LazyLock<IpcPluginStore> = LazyLock::new(IpcPluginStore::new);

pub struct IpcPluginRegistry;

impl<'p> IpcPluginRegistry {
	pub fn register(process: IpcProcess) -> Result<u64, Error> {
		STORE.insert(process)
	}

	pub fn unregister<P>(process: P) -> Result<(), Error>
	where
		P: Into<IpcPluginId<'p>>,
	{
		match process.into() {
			IpcPluginId::Index(index) => Self::unregister_with_index(index),
			IpcPluginId::Name(name) => Self::unregister_with_name(name.as_ref()),
		}
	}

	pub fn unregister_with_index(index: u64) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if map.remove(&index).is_some() {
			Ok(())
		} else {
			Err(Error::NotFound("IpcProcess".to_string()))
		}
	}

	pub fn unregister_with_name(name: &str) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		let old_len = map.len();
		map.retain(|_, process| process.name() != name);
		if map.len() == old_len {
			Err(Error::NotFound("IpcProcess".to_string()))
		} else {
			Ok(())
		}
	}

	pub fn get<P>(process: P) -> Option<Arc<IpcProcess>>
	where
		P: Into<IpcPluginId<'p>>,
	{
		match process.into() {
			IpcPluginId::Index(index) => Self::get_with_index(index),
			IpcPluginId::Name(name) => Self::get_with_name(name.as_ref()),
		}
	}

	pub fn get_with_index(index: u64) -> Option<Arc<IpcProcess>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	pub fn get_with_name(name: &str) -> Option<Arc<IpcProcess>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().find(|process| process.name() == name).cloned()
	}

	pub fn all() -> Vec<Arc<IpcProcess>> {
		STORE.all()
	}

	pub fn set_state(index: u64, state: ProcessState) -> Result<(), Error> {
		let process = Self::get_with_index(index)
			.ok_or_else(|| Error::NotFound("IpcProcess".to_string()))?;
		process.set_state(state);
		Ok(())
	}
}


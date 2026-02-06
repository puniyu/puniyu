mod store;
mod types;
pub use types::*;

use crate::{Error, Result, SourceType};
use puniyu_types::hook::Hook;
use std::sync::{Arc, LazyLock};
use store::HookStore;

static STORE: LazyLock<HookStore> = LazyLock::new(HookStore::new);

pub struct HookRegistry;

impl HookRegistry {
	pub fn register(source: SourceType, hook: Arc<dyn Hook>) -> Result<u64> {
		let hook = HookInfo { source, builder: hook };
		STORE.insert(hook)
	}
	pub fn unregister<H>(hook: H) -> Result<()>
	where
		H: Into<HookId>,
	{
		let hook = hook.into();
		match hook {
			HookId::Index(index) => Self::unregister_with_index(index),
			HookId::Name(name) => Self::unregister_with_hook_name(name),
			HookId::Source(source) => Self::unregister_with_source(source),
		}
	}

	pub fn unregister_with_index(index: u64) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if let Some(_) = map.remove(&index) {
			Ok(())
		} else {
			Err(Error::NotFound("Hook".to_string()))
		}
	}

	pub fn unregister_with_source(source: SourceType) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		map.retain(|_, h| h.source != source);
		Ok(())
	}
	pub fn unregister_with_hook_name(name: &str) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		map.retain(|_, h| h.builder.name() != name);
		Ok(())
	}

	pub fn get<H>(hook: H) -> Vec<HookInfo>
	where
		H: Into<HookId>,
	{
		let hook = hook.into();
		match hook {
			HookId::Index(index) => Self::get_with_index(index).into_iter().collect(),
			HookId::Name(name) => Self::get_with_hook_name(&name),
			HookId::Source(source) => Self::get_with_source(source),
		}
	}

	pub fn get_with_index(index: u64) -> Option<HookInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}
	pub fn get_with_source(source: SourceType) -> Vec<HookInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|h| h.source == source).cloned().collect()
	}
	pub fn get_with_hook_name(name: &str) -> Vec<HookInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|h| h.builder.name() == name).cloned().collect()
	}
	pub fn all() -> Vec<HookInfo> {
		STORE.all()
	}
}

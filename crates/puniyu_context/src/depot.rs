use crate::{Error, ScopeId};
use std::any::{Any, TypeId, type_name};
use std::collections::HashMap;
use std::sync::RwLock;

struct Entry {
	owner: ScopeId,
	value: Box<dyn Any + Send + Sync>,
}

#[derive(Default)]
pub(crate) struct Depot {
	map: RwLock<HashMap<TypeId, Entry>>,
}

impl Depot {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert<V: Any + Send + Sync>(&self, owner: ScopeId, value: V) -> Result<(), Error> {
		let type_id = TypeId::of::<V>();
		let mut map = self.map.write().expect("poisoned lock");
		if let Some(entry) = map.get(&type_id) {
			return Err(Error::Conflict { capability: type_name::<V>(), scope: entry.owner });
		}
		map.insert(type_id, Entry { owner, value: Box::new(value) });
		Ok(())
	}

	pub fn get<V: Any + Send + Sync + Clone>(&self) -> Option<V> {
		let type_id = TypeId::of::<V>();
		let map = self.map.read().expect("poisoned lock");
		map.get(&type_id).and_then(|entry| entry.value.downcast_ref::<V>()).cloned()
	}

	pub fn contains<V: Any + Send + Sync>(&self) -> bool {
		let type_id = TypeId::of::<V>();
		self.map.read().expect("poisoned lock").contains_key(&type_id)
	}

	pub fn remove<V: Any + Send + Sync>(&self, owner: ScopeId) -> Option<V> {
		let type_id = TypeId::of::<V>();
		let mut map = self.map.write().expect("poisoned lock");
		if map.get(&type_id).is_none_or(|entry| entry.owner != owner) {
			return None;
		}
		map.remove(&type_id).and_then(|entry| entry.value.downcast::<V>().ok().map(|v| *v))
	}

	pub fn remove_scope(&self, owner: ScopeId) {
		self.map.write().expect("poisoned lock").retain(|_, entry| entry.owner != owner);
	}
}

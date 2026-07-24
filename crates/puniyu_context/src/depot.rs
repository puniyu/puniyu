use crate::Error;
use puniyu_registry::Registry;
use std::any::{Any, TypeId, type_name};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};


#[derive(Clone)]
struct Entry {
	value: Arc<dyn Any + Send + Sync>,
}

pub(crate) struct Depot {
	entries: Registry<Entry>,
	type_index: RwLock<HashMap<TypeId, u64>>,
}

impl Depot {
	pub fn new() -> Self {
		Self {
			entries: Registry::default(),
			type_index: RwLock::new(HashMap::new()),
		}
	}

	pub fn insert<V: Any + Send + Sync>(&self, value: V) -> Result<(), Error> {
		let type_id = TypeId::of::<V>();
		let mut index = self.type_index.write().expect("poisoned lock");
		if index.contains_key(&type_id) {
			return Err(Error::Conflict { capability: type_name::<V>() });
		}
		let id = self.entries.insert(Entry { value: Arc::new(value) });
		index.insert(type_id, id);
		Ok(())
	}

	pub fn get<V: Any + Send + Sync + Clone>(&self) -> Option<V> {
		let type_id = TypeId::of::<V>();
		let index = self.type_index.read().expect("poisoned lock");
		index.get(&type_id).and_then(|&id| {
			self.entries.get(id).and_then(|e| e.value.downcast_ref::<V>().cloned())
		})
	}

	pub fn contains<V: Any + Send + Sync>(&self) -> bool {
		self.type_index.read().expect("poisoned lock").contains_key(&TypeId::of::<V>())
	}

	pub fn remove<V: Any + Send + Sync>(&self) -> Option<V> {
		let type_id = TypeId::of::<V>();
		let mut index = self.type_index.write().expect("poisoned lock");
		index.remove(&type_id).and_then(|id| {
			self.entries.remove(id).and_then(|e| {
				e.value.downcast::<V>().ok().and_then(|arc| Arc::try_unwrap(arc).ok())
			})
		})
	}
}

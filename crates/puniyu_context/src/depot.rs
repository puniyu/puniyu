use crate::Error;
use puniyu_registry::Registry;
use std::any::{Any, TypeId, type_name};
use std::sync::Arc;


#[derive(Clone)]
struct Entry {
	value: Arc<dyn Any + Send + Sync>,
}

#[derive(Default)]
pub(crate) struct Depot {
	entries: Registry<Entry>,
	type_index: scc::HashMap<TypeId, u64>,
}

#[allow(dead_code)]
impl Depot {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert<V: Any + Send + Sync>(&self, value: V) -> Result<(), Error> {
		let type_id = TypeId::of::<V>();
		if self.type_index.contains_sync(&type_id) {
			return Err(Error::Conflict { capability: type_name::<V>() });
		}
		let id = self.entries.insert(Entry { value: Arc::new(value) });
		self.type_index.insert_sync(type_id, id).ok();
		Ok(())
	}

	pub fn get<V: Any + Send + Sync + Clone>(&self) -> Option<V> {
		let type_id = TypeId::of::<V>();
		self.type_index.read_sync(&type_id, |_, &id| id).and_then(|id| {
			self.entries.get(id).and_then(|e| e.value.downcast_ref::<V>().cloned())
		})
	}

	pub fn contains<V: Any + Send + Sync>(&self) -> bool {
		self.type_index.contains_sync(&TypeId::of::<V>())
	}

	pub fn remove<V: Any + Send + Sync>(&self) -> Option<V> {
		let type_id = TypeId::of::<V>();
		self.type_index.remove_sync(&type_id).and_then(|(_, id)| {
			self.entries.remove(id).and_then(|e| {
				e.value.downcast::<V>().ok().and_then(|arc| Arc::try_unwrap(arc).ok())
			})
		})
	}
}

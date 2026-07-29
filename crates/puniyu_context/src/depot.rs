#![allow(dead_code)]

use crate::Error;
use puniyu_service::Service;
use smol_str::SmolStr;
use std::any::{Any, TypeId};
use std::sync::Arc;


#[derive(Default, Clone)]
pub(crate) struct NamedDepot {
	services: scc::HashMap<SmolStr, Arc<dyn Service>>,
}

impl NamedDepot {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert(&self, service: Arc<dyn Service>) -> Result<(), Error> {
		let name = SmolStr::new(service.name());
		if self.services.contains_sync(&name) {
			return Err(Error::Conflict { name });
		}
		self.services.insert_sync(name, service).ok();
		Ok(())
	}

	pub fn get(&self, name: &str) -> Option<Arc<dyn Service>> {
		self.services.read_sync(&SmolStr::new(name), |_, v| v.clone())
	}

	pub fn contains(&self, name: &str) -> bool {
		self.services.contains_sync(&SmolStr::new(name))
	}
}


#[derive(Default, Clone)]
pub(crate) struct TypedDepot {
	services: scc::HashMap<TypeId, Arc<dyn Service>>,
}

impl TypedDepot {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert<S: Service>(&self, service: S) {
		let service = Arc::new(service);
		let type_id = service.as_ref().type_id();
		self.services.insert_sync(type_id, service).ok();
	}

	pub fn get<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
		self.services.read_sync(&TypeId::of::<T>(), |_, v| v.clone()).and_then(|s| downcast(s))
	}

	pub fn contains<T: Any>(&self) -> bool {
		self.services.contains_sync(&TypeId::of::<T>())
	}
}

fn downcast<T: Any + Send + Sync>(service: Arc<dyn Service>) -> Option<Arc<T>> {
	if service.as_ref().type_id() == TypeId::of::<T>() {
		Some(unsafe { Arc::from_raw(Arc::into_raw(service) as *const T) })
	} else {
		None
	}
}

use std::{any::Any, sync::Arc};

use puniyu_service::Service;

use crate::depot::TypedDepot;

pub struct ServiceContext {
	pub(crate) inner: TypedDepot,
}

impl ServiceContext {
    pub(crate) fn new() -> Self {
		Self { inner: TypedDepot::new() }
	}

	/// 注入服务
	pub fn inject<S: Service>(&self, service: S) {
		self.inner.insert(service);
	}

	/// 注入已类型擦除的服务（用于批量注入 Arc<dyn Service>）
	pub fn inject_dyn(&self, service: Arc<dyn Service>) {
		self.inner.insert_dyn(service);
	}

	/// 按类型查找服务
	pub fn require<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
		self.inner.get::<T>()
	}
}

use std::any::Any;
use std::sync::Arc;
use puniyu_action::{Action, ActionRegistry};
use puniyu_service::Service;

use crate::depot::TypedDepot;

pub struct PluginContext {
	pub(crate) depot: TypedDepot,
	pub(crate) actions: ActionRegistry,
    pub(crate) path: puniyu_path::Path,
}

impl PluginContext {
    pub fn path(&self) -> &puniyu_path::Path {
		&self.path
	}
	/// 注册 Action。
	pub fn action<A: Action + 'static>(&self, action: A) {
		self.actions.insert(action);
	}

	/// 注入服务
	pub fn inject(&self, service: Arc<dyn Service>) {
		self.depot.insert(service);
	}

	/// 按类型查找服务。
	pub fn require<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
		self.depot.get::<T>()
	}
}

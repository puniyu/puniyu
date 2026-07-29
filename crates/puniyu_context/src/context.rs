mod plugin;
pub use plugin::PluginContext;
mod adapter;
pub use adapter::AdapterContext;
use puniyu_service::Service;

use crate::depot::{NamedDepot, TypedDepot};
use crate::Error;
use puniyu_action::ActionRegistry;
use std::sync::Arc;


pub struct Context {
	depot: NamedDepot,
	path: puniyu_path::Path
}

impl Context {
	/// 创建根上下文。
	pub fn new(path: puniyu_path::Path) -> Self {
		Self {
			depot: NamedDepot::new(),
			path
		}
	}

	/// 注入服务
	pub fn inject<S: Service>(&self, service: S) -> Result<(), Error> {
		self.depot.insert(Arc::new(service))
	}


	/// 查找服务
	pub fn require<S: Service>(&self, service: S) -> Option<Arc<dyn Service>> {
		self.depot.get(service.name())
	}

	/// 检查服务是否存在
	pub fn contains(&self, name: &str) -> bool {
		self.depot.contains(name)
	}

	/// 创建插件子上下文
	pub fn plugin(&self) -> PluginContext {
		PluginContext {
			depot: TypedDepot::new(),
			actions: ActionRegistry::new(),
			path: self.path.clone(),
		}
	}

	/// 创建适配器子上下文
	pub fn adapter(&self) -> AdapterContext {
		AdapterContext {
			path: self.path.clone(),
		}
	}

	/// 获取路径
	pub fn path(&self) -> &puniyu_path::Path {
		&self.path
	}
}

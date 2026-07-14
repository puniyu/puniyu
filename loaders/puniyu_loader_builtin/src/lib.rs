//! # puniyu_loader_builtin
//!
//! Puniyu 内置加载器，用于在编译期通过构建器模式注册 adapter 和 plugin。
//!
//! ## 使用方式
//!
//! ```rust,ignore
//! use puniyu_loader_builtin::BuiltinLoader;
//!
//! let loader = BuiltinLoader::new()
//!     .with_adapter(MyAdapter)
//!     .with_plugin(MyPlugin);
//! ```

use async_trait::async_trait;
use puniyu_adapter_core::Adapter;
use puniyu_error::AnyError;
use puniyu_loader::Loader;
use puniyu_plugin_core::Plugin;
use std::sync::Arc;

#[derive(Default)]
pub struct BuiltinLoader {
	adapters: Vec<Arc<dyn Adapter>>,
	plugins: Vec<Arc<dyn Plugin>>,
}

impl BuiltinLoader {
	#[inline]
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_adapter(mut self, adapter: impl Adapter + 'static) -> Self {
		self.adapters.push(Arc::new(adapter));
		self
	}

	pub fn with_plugin(mut self, plugin: impl Plugin + 'static) -> Self {
		self.plugins.push(Arc::new(plugin));
		self
	}
}

#[async_trait]
impl Loader for BuiltinLoader {
	fn name(&self) -> &str {
		"builtin"
	}

	async fn adapters(&self) -> AnyError<Vec<Arc<dyn Adapter>>> {
		Ok(self.adapters.clone())
	}

	async fn plugins(&self) -> AnyError<Vec<Arc<dyn Plugin>>> {
		Ok(self.plugins.clone())
	}
}

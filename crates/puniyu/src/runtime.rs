use puniyu_adapter_core::Adapter;
use puniyu_context::{AdapterContext, Context, PluginContext, ServiceContext};
use puniyu_plugin_core::Plugin;
use puniyu_service::Service;
use std::cmp::Reverse;
use std::sync::Arc;

struct Entry<T, C> {
	component: T,
	context: C,
}

pub(crate) struct Runtime<T, C> {
	app_context: Arc<Context>,
	components: Vec<Entry<T, C>>,
}

impl<T, C> Runtime<T, C> {
	pub fn new(app_context: Arc<Context>) -> Self {
		Self { app_context, components: Vec::new() }
	}
}


impl Runtime<Arc<dyn Service>, ServiceContext> {
	pub fn add(&mut self, component: Arc<dyn Service>) {
		let context = ServiceContext::new(Arc::clone(&self.app_context), component.name());
		self.components.push(Entry { component, context });
	}

	pub async fn setup(&self) {
		for entry in &self.components {
			if let Err(e) = entry.component.setup(&entry.context).await {
				log::error!("service {} setup failed: {e}", entry.component.name());
			}
		}
	}

	pub async fn cleanup(&self) {
		for entry in self.components.iter().rev() {
			if let Err(e) = entry.component.cleanup(&entry.context).await {
				log::error!("service {} cleanup failed: {e}", entry.component.name());
			}
		}
	}
}

impl Runtime<Arc<dyn Plugin>, PluginContext> {
	pub fn add(&mut self, component: Arc<dyn Plugin>) {
		let context = PluginContext::new(Arc::clone(&self.app_context), component.name());
		self.components.push(Entry { component, context });
	}

	async fn sorted(&self) -> Vec<&Entry<Arc<dyn Plugin>, PluginContext>> {
		let mut sorted: Vec<_> = self.components.iter().collect();
		sorted.sort_by_key(|e| e.component.priority());
		sorted
	}

	async fn sorted_rev(&self) -> Vec<&Entry<Arc<dyn Plugin>, PluginContext>> {
		let mut sorted: Vec<_> = self.components.iter().collect();
		sorted.sort_by_key(|e| Reverse(e.component.priority()));
		sorted
	}

	pub async fn start(&self) {
		for entry in self.sorted().await {
			if let Err(e) = entry.component.on_start(&entry.context).await {
				log::error!("plugin {} start failed: {e}", entry.component.name());
			}
		}
	}

	pub async fn load(&self) {
		for entry in self.sorted().await {
			if let Err(e) = entry.component.on_load(&entry.context).await {
				log::error!("plugin {} load failed: {e}", entry.component.name());
			}
		}
	}

	pub async fn unload(&self) {
		for entry in self.sorted_rev().await {
			if let Err(e) = entry.component.on_unload(&entry.context).await {
				log::error!("plugin {} unload failed: {e}", entry.component.name());
			}
		}
	}

	pub async fn stop(&self) {
		for entry in self.sorted_rev().await {
			if let Err(e) = entry.component.on_stop(&entry.context).await {
				log::error!("plugin {} stop failed: {e}", entry.component.name());
			}
		}
	}
}


impl Runtime<Arc<dyn Adapter>, AdapterContext> {
	pub fn add(&mut self, component: Arc<dyn Adapter>) {
		let name = component.name();
		let context = AdapterContext::new(Arc::clone(&self.app_context), name);
		let _ = context.insert(Arc::clone(&component));
		self.components.push(Entry { component, context });
	}

	async fn sorted(&self) -> Vec<&Entry<Arc<dyn Adapter>, AdapterContext>> {
		let mut sorted: Vec<_> = self.components.iter().collect();
		sorted.sort_by_key(|e| e.component.priority());
		sorted
	}

	async fn sorted_rev(&self) -> Vec<&Entry<Arc<dyn Adapter>, AdapterContext>> {
		let mut sorted: Vec<_> = self.components.iter().collect();
		sorted.sort_by_key(|e| Reverse(e.component.priority()));
		sorted
	}

	pub async fn start(&self) {
		for entry in self.sorted().await {
			if let Err(e) = entry.component.on_start(&entry.context).await {
				log::error!("adapter {} start failed: {e}", entry.component.name());
			}
		}
	}

	pub async fn load(&self) {
		for entry in self.sorted().await {
			if let Err(e) = entry.component.on_load(&entry.context).await {
				log::error!("adapter {} load failed: {e}", entry.component.name());
			}
		}
	}

	pub async fn unload(&self) {
		for entry in self.sorted_rev().await {
			if let Err(e) = entry.component.on_unload(&entry.context).await {
				log::error!("adapter {} unload failed: {e}", entry.component.name());
			}
		}
	}

	pub async fn stop(&self) {
		for entry in self.sorted_rev().await {
			if let Err(e) = entry.component.on_stop(&entry.context).await {
				log::error!("adapter {} stop failed: {e}", entry.component.name());
			}
		}
	}
}

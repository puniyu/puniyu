use log::debug;
use puniyu_adapter_core::Adapter;
use puniyu_context::{AdapterContext, AppContext};
use std::sync::Arc;

struct AdapterInstance {
	adapter: Arc<dyn Adapter>,
	context: AdapterContext,
	started: bool,
	loaded: bool,
}

pub(crate) struct AdapterRuntime {
	app_context: Arc<AppContext>,
	adapters: Vec<AdapterInstance>,
}

impl AdapterRuntime {
	pub(crate) fn new(app_context: Arc<AppContext>, adapters: Vec<Arc<dyn Adapter>>) -> Self {
		let mut adapters: Vec<AdapterInstance> = adapters
			.into_iter()
			.map(|adapter| {
				let info = adapter.adapter_info();
				let context = AdapterContext::new(Arc::clone(&app_context), info.name);
				AdapterInstance { adapter, context, started: false, loaded: false }
			})
			.collect();
		adapters.sort_by_key(|i| i.adapter.priority());
		Self { app_context, adapters }
	}

	pub(crate) async fn start(&mut self) {
		for instance in &mut self.adapters {
			let name = instance.adapter.adapter_info().name;
			debug!("adapter '{name}' starting...");
			if let Err(e) = instance.adapter.on_start(&instance.context).await {
				log::error!("adapter '{name}' on_start failed: {e}");
				continue;
			}
			debug!("adapter '{name}' started");
			instance.started = true;
		}
	}

	pub(crate) async fn load(&mut self) {
		for instance in &mut self.adapters {
			if !instance.started {
				continue;
			}
			let name = instance.adapter.adapter_info().name;
			debug!("adapter '{name}' loading...");
			if let Err(e) = instance.adapter.on_load(&instance.context).await {
				log::error!("adapter '{name}' on_load failed: {e}");
				continue;
			}
			debug!("adapter '{name}' loaded");
			instance.loaded = true;
		}
	}

	pub(crate) async fn shutdown(&mut self) {
		for instance in self.adapters.iter_mut().rev() {
			if instance.loaded {
				let name = instance.adapter.adapter_info().name;
				if let Err(e) = instance.adapter.on_unload(&instance.context).await {
					log::error!("adapter '{name}' on_unload failed: {e}");
				}
			}
		}
		for instance in self.adapters.iter_mut().rev() {
			if instance.started {
				let name = instance.adapter.adapter_info().name;
				if let Err(e) = instance.adapter.on_stop(&instance.context).await {
					log::error!("adapter '{name}' on_stop failed: {e}");
				}
			}
		}
		for instance in self.adapters.iter().rev() {
			self.app_context.remove_scope(instance.context.scope_id());
		}
	}
}

use log::debug;
use puniyu_context::{AppContext, PluginContext};
use puniyu_plugin_core::Plugin;
use std::sync::Arc;

struct PluginInstance {
	plugin: Arc<dyn Plugin>,
	context: PluginContext,
	started: bool,
	loaded: bool,
}

pub(crate) struct PluginRuntime {
	app_context: Arc<AppContext>,
	plugins: Vec<PluginInstance>,
}

impl PluginRuntime {
	pub(crate) fn new(app_context: Arc<AppContext>, plugins: Vec<Arc<dyn Plugin>>) -> Self {
		let mut plugins: Vec<PluginInstance> = plugins
			.into_iter()
			.map(|plugin| {
				let context = PluginContext::new(Arc::clone(&app_context), plugin.name());
				PluginInstance { plugin, context, started: false, loaded: false }
			})
			.collect();
		plugins.sort_by_key(|i| i.plugin.priority());
		Self { app_context, plugins }
	}

	pub(crate) async fn start(&mut self) {
		for instance in &mut self.plugins {
			let name = instance.plugin.name();
			debug!("plugin '{name}' starting...");
			if let Err(e) = instance.plugin.on_start(&instance.context).await {
				log::error!("plugin '{name}' on_start failed: {e}");
				continue;
			}
			debug!("plugin '{name}' started");
			instance.started = true;
		}
	}

	pub(crate) async fn load(&mut self) {
		for instance in &mut self.plugins {
			if !instance.started {
				continue;
			}
			let name = instance.plugin.name();
			debug!("plugin '{name}' loading...");
			if let Err(e) = instance.plugin.on_load(&instance.context).await {
				log::error!("plugin '{name}' on_load failed: {e}");
				continue;
			}
			debug!("plugin '{name}' loaded");
			instance.loaded = true;
		}
	}

	pub(crate) async fn shutdown(&mut self) {
		for instance in self.plugins.iter_mut().rev() {
			if instance.loaded {
				let name = instance.plugin.name();
				if let Err(e) = instance.plugin.on_unload(&instance.context).await {
					log::error!("plugin '{name}' on_unload failed: {e}");
				}
			}
		}
		for instance in self.plugins.iter_mut().rev() {
			if instance.started {
				let name = instance.plugin.name();
				if let Err(e) = instance.plugin.on_stop(&instance.context).await {
					log::error!("plugin '{name}' on_stop failed: {e}");
				}
			}
		}
		for instance in self.plugins.iter().rev() {
			self.app_context.remove_scope(instance.context.scope_id());
		}
	}
}

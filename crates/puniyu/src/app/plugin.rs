use super::{AppError, PluginLifecycleFailure, PluginLifecyclePhase};
use puniyu_context::{AppContext, PluginContext};
use puniyu_plugin_core::Plugin;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginState {
	Discovered,
	Started,
	Loaded,
	Unloaded,
	Stopped,
	Failed,
}

struct PluginInstance {
	plugin: Arc<dyn Plugin>,
	context: PluginContext,
	state: PluginState,
	started: bool,
	loaded: bool,
}

pub struct PluginRuntime {
	app_context: Arc<AppContext>,
	plugins: Vec<PluginInstance>,
}

impl PluginRuntime {
	pub fn new(
		app_context: Arc<AppContext>,
		plugins: Vec<Arc<dyn Plugin>>,
	) -> Result<Self, AppError> {
		let plugins = Self::sort_plugins(plugins)?
			.into_iter()
			.map(|plugin| {
				let context = PluginContext::new(
					Arc::clone(&app_context),
					app_context.new_scope(),
					plugin.name(),
				);
				PluginInstance {
					plugin,
					context,
					state: PluginState::Discovered,
					started: false,
					loaded: false,
				}
			})
			.collect();
		Ok(Self { app_context, plugins })
	}

	pub async fn start(&mut self) -> Result<(), AppError> {
		for index in 0..self.plugins.len() {
			let result = {
				let instance = &self.plugins[index];
				instance.plugin.on_start(&instance.context).await
			};
			match result {
				Ok(()) => {
					self.plugins[index].started = true;
					self.plugins[index].state = PluginState::Started;
				}
				Err(source) => {
					self.plugins[index].state = PluginState::Failed;
					let plugin = self.plugins[index].plugin.name().to_string();
					let rollback_failures = self.stop_started().await;
					self.cleanup_scopes();
					return Err(AppError::PluginStart { plugin, source, rollback_failures });
				}
			}
		}
		Ok(())
	}

	pub async fn load(&mut self) -> Result<(), AppError> {
		for index in 0..self.plugins.len() {
			if !self.plugins[index].started {
				continue;
			}
			let result = {
				let instance = &self.plugins[index];
				instance.plugin.on_load(&instance.context).await
			};
			match result {
				Ok(()) => {
					self.plugins[index].loaded = true;
					self.plugins[index].state = PluginState::Loaded;
				}
				Err(source) => {
					self.plugins[index].state = PluginState::Failed;
					let plugin = self.plugins[index].plugin.name().to_string();
					let mut rollback_failures = self.unload_loaded().await;
					rollback_failures.extend(self.stop_started().await);
					self.cleanup_scopes();
					return Err(AppError::PluginLoad { plugin, source, rollback_failures });
				}
			}
		}
		Ok(())
	}

	pub async fn shutdown(&mut self) -> Result<(), AppError> {
		let mut failures = self.unload_loaded().await;
		failures.extend(self.stop_started().await);
		self.cleanup_scopes();
		if failures.is_empty() { Ok(()) } else { Err(AppError::PluginShutdown(failures)) }
	}

	async fn stop_started(&mut self) -> Vec<PluginLifecycleFailure> {
		let mut failures = Vec::new();
		for index in (0..self.plugins.len()).rev() {
			if !self.plugins[index].started {
				continue;
			}
			let result = {
				let instance = &self.plugins[index];
				instance.plugin.on_stop(&instance.context).await
			};
			self.plugins[index].started = false;
			match result {
				Ok(()) => self.plugins[index].state = PluginState::Stopped,
				Err(error) => {
					self.plugins[index].state = PluginState::Failed;
					failures.push(PluginLifecycleFailure {
						plugin: self.plugins[index].plugin.name().to_string(),
						phase: PluginLifecyclePhase::Stop,
						message: error.to_string(),
					});
				}
			}
		}
		failures
	}

	async fn unload_loaded(&mut self) -> Vec<PluginLifecycleFailure> {
		let mut failures = Vec::new();
		for index in (0..self.plugins.len()).rev() {
			if !self.plugins[index].loaded {
				continue;
			}
			let result = {
				let instance = &self.plugins[index];
				instance.plugin.on_unload(&instance.context).await
			};
			self.plugins[index].loaded = false;
			match result {
				Ok(()) => self.plugins[index].state = PluginState::Unloaded,
				Err(error) => {
					self.plugins[index].state = PluginState::Failed;
					failures.push(PluginLifecycleFailure {
						plugin: self.plugins[index].plugin.name().to_string(),
						phase: PluginLifecyclePhase::Unload,
						message: error.to_string(),
					});
				}
			}
		}
		failures
	}

	fn cleanup_scopes(&mut self) {
		for instance in self.plugins.iter().rev() {
			self.app_context.remove_scope(instance.context.scope_id());
		}
	}

	fn sort_plugins(plugins: Vec<Arc<dyn Plugin>>) -> Result<Vec<Arc<dyn Plugin>>, AppError> {
		let mut names = HashSet::with_capacity(plugins.len());
		for plugin in &plugins {
			let name = plugin.name().to_string();
			if !names.insert(name.clone()) {
				return Err(AppError::DuplicatePlugin(name));
			}
			let required = plugin.required_version();
			if !required.matches(&puniyu_version::VERSION) {
				return Err(AppError::IncompatiblePluginVersion {
					plugin: name,
					required: required.to_string(),
					current: puniyu_version::VERSION.to_string(),
				});
			}
		}
		let mut indexed = plugins.into_iter().enumerate().collect::<Vec<_>>();
		indexed.sort_by_key(|(index, plugin)| (plugin.priority(), *index));
		Ok(indexed.into_iter().map(|(_, plugin)| plugin).collect())
	}
}

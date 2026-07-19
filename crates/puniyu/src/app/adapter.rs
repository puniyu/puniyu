use super::{AdapterLifecycleFailure, AdapterLifecyclePhase};
use puniyu_adapter_core::Adapter;
use puniyu_context::{AdapterContext, AppContext};
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterState {
	Discovered,
	Started,
	Loaded,
	Unloaded,
	Stopped,
	Failed,
}

struct AdapterInstance {
	adapter: Arc<dyn Adapter>,
	context: AdapterContext,
	name: String,
	discovery_index: usize,
	state: AdapterState,
	valid: bool,
	started: bool,
	loaded: bool,
}

pub struct AdapterRuntime {
	app_context: Arc<AppContext>,
	adapters: Vec<AdapterInstance>,
	validation_failures: Vec<AdapterLifecycleFailure>,
}

impl AdapterRuntime {
	pub fn new(app_context: Arc<AppContext>, adapters: Vec<Arc<dyn Adapter>>) -> Self {
		let mut names = HashSet::with_capacity(adapters.len());
		let mut validation_failures = Vec::new();
		let mut adapters = adapters
			.into_iter()
			.enumerate()
			.map(|(discovery_index, adapter)| {
				let info = adapter.adapter_info();
				let name = info.name.to_string();
				let context = AdapterContext::new(
					Arc::clone(&app_context),
					app_context.new_scope(),
					info.name,
				);
				let mut instance = AdapterInstance {
					adapter,
					context,
					name: name.clone(),
					discovery_index,
					state: AdapterState::Discovered,
					valid: true,
					started: false,
					loaded: false,
				};

				let message = if !names.insert(name.clone()) {
					Some("duplicate adapter name".to_string())
				} else {
					let required = instance.adapter.required_version();
					(!required.matches(&puniyu_version::VERSION)).then(|| {
						format!(
							"requires core version '{required}', current version is '{}'",
							puniyu_version::VERSION
						)
					})
				};
				if let Some(message) = message {
					instance.valid = false;
					instance.state = AdapterState::Failed;
					app_context.remove_scope(instance.context.scope_id());
					validation_failures.push(AdapterLifecycleFailure {
						adapter: name,
						phase: AdapterLifecyclePhase::Validation,
						message,
					});
				}
				instance
			})
			.collect::<Vec<_>>();

		adapters.sort_by_key(|instance| (instance.adapter.priority(), instance.discovery_index));
		Self { app_context, adapters, validation_failures }
	}

	pub async fn start(&mut self) -> Vec<AdapterLifecycleFailure> {
		let mut failures = std::mem::take(&mut self.validation_failures);
		for index in 0..self.adapters.len() {
			if !self.adapters[index].valid {
				continue;
			}
			let result = {
				let instance = &self.adapters[index];
				instance.adapter.on_start(&instance.context).await
			};
			match result {
				Ok(()) => {
					self.adapters[index].started = true;
					self.adapters[index].state = AdapterState::Started;
				}
				Err(error) => {
					self.adapters[index].state = AdapterState::Failed;
					self.adapters[index].valid = false;
					self.app_context.remove_scope(self.adapters[index].context.scope_id());
					failures.push(self.failure(
						index,
						AdapterLifecyclePhase::Start,
						error.to_string(),
					));
				}
			}
		}
		failures
	}

	pub async fn load(&mut self) -> Vec<AdapterLifecycleFailure> {
		let mut failures = Vec::new();
		for index in 0..self.adapters.len() {
			if !self.adapters[index].started || !self.adapters[index].valid {
				continue;
			}
			let result = {
				let instance = &self.adapters[index];
				instance.adapter.on_load(&instance.context).await
			};
			match result {
				Ok(()) => {
					self.adapters[index].loaded = true;
					self.adapters[index].state = AdapterState::Loaded;
				}
				Err(error) => {
					self.adapters[index].state = AdapterState::Failed;
					failures.push(self.failure(
						index,
						AdapterLifecyclePhase::Load,
						error.to_string(),
					));
					failures.extend(self.stop_one(index).await);
					self.adapters[index].valid = false;
					self.adapters[index].state = AdapterState::Failed;
					self.app_context.remove_scope(self.adapters[index].context.scope_id());
				}
			}
		}
		failures
	}

	pub async fn shutdown(&mut self) -> Vec<AdapterLifecycleFailure> {
		let mut failures = self.unload_loaded().await;
		failures.extend(self.stop_started().await);
		for instance in self.adapters.iter().rev() {
			self.app_context.remove_scope(instance.context.scope_id());
		}
		failures
	}

	async fn stop_started(&mut self) -> Vec<AdapterLifecycleFailure> {
		let mut failures = Vec::new();
		for index in (0..self.adapters.len()).rev() {
			failures.extend(self.stop_one(index).await);
		}
		failures
	}

	async fn stop_one(&mut self, index: usize) -> Vec<AdapterLifecycleFailure> {
		if !self.adapters[index].started {
			return Vec::new();
		}
		let result = {
			let instance = &self.adapters[index];
			instance.adapter.on_stop(&instance.context).await
		};
		self.adapters[index].started = false;
		match result {
			Ok(()) => {
				self.adapters[index].state = AdapterState::Stopped;
				Vec::new()
			}
			Err(error) => {
				self.adapters[index].state = AdapterState::Failed;
				vec![self.failure(index, AdapterLifecyclePhase::Stop, error.to_string())]
			}
		}
	}

	async fn unload_loaded(&mut self) -> Vec<AdapterLifecycleFailure> {
		let mut failures = Vec::new();
		for index in (0..self.adapters.len()).rev() {
			failures.extend(self.unload_one(index).await);
		}
		failures
	}

	async fn unload_one(&mut self, index: usize) -> Vec<AdapterLifecycleFailure> {
		if !self.adapters[index].loaded {
			return Vec::new();
		}
		let result = {
			let instance = &self.adapters[index];
			instance.adapter.on_unload(&instance.context).await
		};
		self.adapters[index].loaded = false;
		match result {
			Ok(()) => {
				self.adapters[index].state = AdapterState::Unloaded;
				Vec::new()
			}
			Err(error) => {
				self.adapters[index].state = AdapterState::Failed;
				vec![self.failure(index, AdapterLifecyclePhase::Unload, error.to_string())]
			}
		}
	}

	fn failure(
		&self,
		index: usize,
		phase: AdapterLifecyclePhase,
		message: String,
	) -> AdapterLifecycleFailure {
		AdapterLifecycleFailure { adapter: self.adapters[index].name.clone(), phase, message }
	}
}

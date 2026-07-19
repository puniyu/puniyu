mod adapter;
mod error;
mod plugin;

pub use adapter::AdapterState;
pub use error::{
	AdapterLifecycleFailure, AdapterLifecyclePhase, AppError, PluginLifecycleFailure,
	PluginLifecyclePhase,
};
pub use plugin::PluginState;

use bon::Builder;
use convert_case::{Case, Casing};
use log::{debug, info};
use puniyu_context::AppContext;
use puniyu_logger::owo_colors::OwoColorize;
use std::sync::Arc;
use std::time::{Duration, Instant};

type BoxFuture = std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>;
type AsyncFn = Box<dyn Fn() -> BoxFuture + Send + Sync>;

#[derive(Builder)]
pub struct App {
	#[builder(field)]
	loaders: Vec<Box<dyn puniyu_loader::Loader>>,
	#[builder(field)]
	on_start: Option<AsyncFn>,
	#[builder(field)]
	on_exit: Option<AsyncFn>,
	#[builder(default = "puniyu")]
	name: &'static str,
}

impl<S: app_builder::State> AppBuilder<S> {
	pub fn loader(mut self, loader: impl puniyu_loader::Loader) -> Self {
		self.loaders.push(Box::new(loader));
		self
	}
	pub fn on_start<F, Fut>(mut self, f: F) -> Self
	where
		F: Fn() -> Fut + Send + Sync + 'static,
		Fut: std::future::Future<Output = ()> + Send + 'static,
	{
		self.on_start = Some(Box::new(move || Box::pin(f())));
		self
	}

	pub fn on_exit<F, Fut>(mut self, f: F) -> Self
	where
		F: Fn() -> Fut + Send + Sync + 'static,
		Fut: std::future::Future<Output = ()> + Send + 'static,
	{
		self.on_exit = Some(Box::new(move || Box::pin(f())));
		self
	}
}

impl App {
	pub async fn run(self) -> Result<(), AppError> {
		let Self { loaders, on_start, on_exit, name } = self;
		let start_time = Instant::now();
		if let Some(callback) = on_start {
			(callback)().await;
		}

		let app_ctx = Arc::new(AppContext::new());

		let mut discovered_adapters = Vec::new();
		let mut discovered_plugins = Vec::new();
		for loader in loaders {
			let loader_name = loader.name().to_string();
			debug!("components discovering from loader '{loader_name}'...");
			let adapters = loader.adapters().await.map_err(|source| AppError::LoaderDiscovery {
				loader: loader_name.clone(),
				component: "adapters",
				source,
			})?;
			discovered_adapters.extend(adapters);
			let plugins = loader.plugins().await.map_err(|source| AppError::LoaderDiscovery {
				loader: loader_name.clone(),
				component: "plugins",
				source,
			})?;
			discovered_plugins.extend(plugins);
			debug!("components discovered from loader '{loader_name}'");
		}

		let mut plugin_runtime =
			plugin::PluginRuntime::new(Arc::clone(&app_ctx), discovered_plugins)?;
		plugin_runtime.start().await?;
		plugin_runtime.load().await?;

		let mut adapter_runtime =
			adapter::AdapterRuntime::new(Arc::clone(&app_ctx), discovered_adapters);
		for failure in adapter_runtime.start().await {
			log_adapter_failure(&failure);
		}
		for failure in adapter_runtime.load().await {
			log_adapter_failure(&failure);
		}

		info!(
			"{} initialized in {}",
			name.fg_rgb::<64, 224, 208>(),
			format_duration(start_time.elapsed()).fg_rgb::<255, 127, 80>()
		);

		let mut primary_error = tokio::signal::ctrl_c().await.err().map(AppError::Io);
		info!("Puniyu stopping...");

		let adapter_failures = adapter_runtime.shutdown().await;
		if !adapter_failures.is_empty() {
			let error = AppError::AdapterShutdown(adapter_failures);
			if primary_error.is_none() {
				primary_error = Some(error);
			} else {
				log::error!("failed to shutdown adapters: {error}");
			}
		}
		if let Err(error) = plugin_runtime.shutdown().await {
			if primary_error.is_none() {
				primary_error = Some(error);
			} else {
				log::error!("failed to shutdown plugins: {error}");
			}
		}
		if let Some(callback) = on_exit {
			(callback)().await;
		}
		let uptime = start_time.elapsed();
		info!(
			"{} uptime: {}",
			name.to_case(Case::Lower).fg_rgb::<64, 224, 208>(),
			format_duration(uptime).fg_rgb::<255, 127, 80>()
		);
		match primary_error {
			Some(error) => Err(error),
			None => Ok(()),
		}
	}
}

fn log_adapter_failure(failure: &AdapterLifecycleFailure) {
	log::error!("adapter '{}' {} failed: {}", failure.adapter, failure.phase, failure.message);
}
fn format_duration(duration: Duration) -> String {
	let mins = duration.as_secs() / 60;
	let secs = duration.as_secs() % 60;
	let ms = duration.subsec_millis();

	match (mins, secs, ms) {
		(0, 0, _) => format!("{ms}ms"),
		(0, _, _) if ms == 0 => format!("{secs}s"),
		(0, _, _) => format!("{}s", secs as f64 + ms as f64 / 1000.0),
		(_, _, _) => format!("{mins}m {secs}s"),
	}
}

use crate::runtime::{AdapterRuntime, PluginRuntime, ServiceRuntime};
use bon::Builder;
use convert_case::{Case, Casing};
use log::info;
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
	pub async fn run(self) -> Result<(), std::io::Error> {
		let Self { loaders, on_start, on_exit, name } = self;
		let start_time = Instant::now();

		if let Some(callback) = on_start {
			(callback)().await;
		}

		let app_ctx = Arc::new(AppContext::new());

		let mut discovered_services = Vec::new();
		let mut discovered_adapters = Vec::new();
		let mut discovered_plugins = Vec::new();
		for loader in loaders {
			let loader_name = loader.name().to_string();
			info!("discovering components from loader '{loader_name}'...");
			match loader.services().await {
				Ok(services) => {
					info!("  found {} service(s)", services.len());
					discovered_services.extend(services);
				}
				Err(e) => log::error!("  failed to discover services: {e}"),
			}
			match loader.adapters().await {
				Ok(adapters) => {
					info!("  found {} adapter(s)", adapters.len());
					discovered_adapters.extend(adapters);
				}
				Err(e) => log::error!("  failed to discover adapters: {e}"),
			}
			match loader.plugins().await {
				Ok(plugins) => {
					info!("  found {} plugin(s)", plugins.len());
					discovered_plugins.extend(plugins);
				}
				Err(e) => log::error!("  failed to discover plugins: {e}"),
			}
		}

		info!(
			"starting {} service(s), {} plugin(s), {} adapter(s)...",
			discovered_services.len(),
			discovered_plugins.len(),
			discovered_adapters.len()
		);

		let mut service_runtime = ServiceRuntime::new(Arc::clone(&app_ctx), discovered_services);
		service_runtime.start().await;

		// Phase 2: Plugins (start + load)
		let mut plugin_runtime = PluginRuntime::new(Arc::clone(&app_ctx), discovered_plugins);
		plugin_runtime.start().await;
		plugin_runtime.load().await;

		// Phase 3: Adapters (start + load)
		let mut adapter_runtime = AdapterRuntime::new(Arc::clone(&app_ctx), discovered_adapters);
		adapter_runtime.start().await;
		adapter_runtime.load().await;

		info!(
			"{} initialized in {}",
			name.fg_rgb::<64, 224, 208>(),
			format_duration(start_time.elapsed()).fg_rgb::<255, 127, 80>()
		);

		tokio::signal::ctrl_c().await?;
		info!("shutting down...");

		adapter_runtime.shutdown().await;
		plugin_runtime.shutdown().await;
		service_runtime.shutdown().await;

		if let Some(callback) = on_exit {
			(callback)().await;
		}

		info!(
			"{} uptime: {}",
			name.to_case(Case::Lower).fg_rgb::<64, 224, 208>(),
			format_duration(start_time.elapsed()).fg_rgb::<255, 127, 80>()
		);

		Ok(())
	}
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

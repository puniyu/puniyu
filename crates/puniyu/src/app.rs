use crate::runtime::Runtime;
use bon::Builder;
use convert_case::Casing;
use log::info;
use puniyu_adapter_core::Adapter;
use puniyu_context::{AdapterContext, AppContext, PluginContext, ServiceContext};
use puniyu_logger::owo_colors::OwoColorize;
use puniyu_plugin_core::Plugin;
use puniyu_service::Service;
use std::sync::Arc;
use std::time::{Duration, Instant};

type BoxFuture = std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + Send>>;
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
		Fut: std::future::Future<Output = std::io::Result<()>> + Send + 'static,
	{
		self.on_start = Some(Box::new(move || Box::pin(f())));
		self
	}

	pub fn on_exit<F, Fut>(mut self, f: F) -> Self
	where
		F: Fn() -> Fut + Send + Sync + 'static,
		Fut: std::future::Future<Output = std::io::Result<()>> + Send + 'static,
	{
		self.on_exit = Some(Box::new(move || Box::pin(f())));
		self
	}
}

impl App {
	pub async fn run(self) -> Result<(), std::io::Error> {
		let Self { loaders, on_start, on_exit, name } = self;
		let start_time = Instant::now();

		if let Some(cb) = on_start {
			(cb)().await?;
		}

		let ctx = Arc::new(AppContext::new());

		let mut services: Runtime<Arc<dyn Service>, ServiceContext> = Runtime::new(Arc::clone(&ctx));
		let mut plugins: Runtime<Arc<dyn Plugin>, PluginContext> = Runtime::new(Arc::clone(&ctx));
		let mut adapters: Runtime<Arc<dyn Adapter>, AdapterContext> = Runtime::new(Arc::clone(&ctx));

		for loader in &loaders {
			if let Ok(s) = loader.services().await {
				for svc in s {
					services.add(svc);
				}
			}
			if let Ok(p) = loader.plugins().await {
				for plug in p {
					plugins.add(plug);
				}
			}
			if let Ok(a) = loader.adapters().await {
				for adp in a {
					adapters.add(adp);
				}
			}
		}

		services.setup().await;
		adapters.start().await;
		plugins.start().await;
		adapters.load().await;
		plugins.load().await;

		info!(
			"{} initialized in {}",
			name.fg_rgb::<64, 224, 208>(),
			format_duration(start_time.elapsed()).fg_rgb::<255, 127, 80>()
		);

		tokio::signal::ctrl_c().await?;
		info!("shutting down...");

		plugins.unload().await;
		adapters.unload().await;
		plugins.stop().await;
		adapters.stop().await;
		services.cleanup().await;

		if let Some(cb) = on_exit {
			(cb)().await?;
		}

		info!(
			"{} uptime: {}",
			name.to_case(convert_case::Case::Lower).fg_rgb::<64, 224, 208>(),
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

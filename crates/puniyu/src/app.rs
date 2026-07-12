mod adapter;
mod plugin;
#[cfg(feature = "server")]
pub(crate) mod server;

use bon::Builder;
use bytes::Bytes;
use convert_case::{Case, Casing};
use log::{debug, info};
use puniyu_dispatch::EventEmitter;
use puniyu_handler::HandlerRegistry;
use puniyu_logger::owo_colors::OwoColorize;
use std::io;
use std::time::{Duration, Instant};

type BoxFuture = std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>;
type AsyncFn = Box<dyn Fn() -> BoxFuture + Send + Sync>;

#[derive(Builder)]
pub struct App {
	#[builder(field)]
	loaders: Vec<Box<dyn puniyu_loader::Loader>>,
	#[builder(field)]
	handlers: Vec<Box<dyn puniyu_handler::Handler>>,
	#[builder(field)]
	#[cfg(feature = "server")]
	routers: Vec<salvo::Router>,
	#[builder(field)]
	#[cfg(feature = "server")]
	hoops: Vec<server::ArcHandler>,
	#[builder(field)]
	on_start: Option<AsyncFn>,
	#[builder(field)]
	on_exit: Option<AsyncFn>,
	#[builder(default = "puniyu")]
	name: &'static str,
    #[cfg(feature = "server")]
    assets: Option<Bytes>
}

impl<S: app_builder::State> AppBuilder<S> {
	pub fn loader(mut self, loader: impl puniyu_loader::Loader) -> Self {
		self.loaders.push(Box::new(loader));
		self
	}
	pub fn handler(mut self, handler: impl puniyu_handler::Handler + 'static) -> Self {
		self.handlers.push(Box::new(handler));
		self
	}

	#[cfg(feature = "server")]
	pub fn router(mut self, router: salvo::Router) -> Self {
		self.routers.push(router);
		self
	}

	#[cfg(feature = "server")]
	pub fn hoop(mut self, hoop: impl salvo::Handler + 'static) -> Self {
		self.hoops.push(server::ArcHandler::new(hoop));
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
	pub async fn run(self) -> io::Result<()> {
		let start_time = Instant::now();
		let cwd_dir = Box::leak(Box::new(std::env::current_dir()?));
		puniyu_app::App::init(self.name, cwd_dir);
		if let Some(callback) = self.on_start {
			(callback)().await;
		}
        info!("{} starting...", self.name.to_case(Case::Lower));
		init_dir().await?;

		debug!("handlers loading...");
		for handler in self.handlers {
			let name = handler.name();
			match HandlerRegistry::register(handler) {
				Ok(_) => debug!("handler '{}' loaded", name),
				Err(e) => log::error!("failed to register handler '{}': {}", name, e),
			}
		}
		debug!("handlers loaded");

		if let Err(e) = puniyu_task::start_scheduler().await {
			log::error!("failed to start task scheduler: {}", e);
		}

		for loader in self.loaders {
			debug!("adapters loading...");
			match loader.adapters().await {
				Ok(adapters) => {
					for adapter in adapters {
						if let Err(e) = adapter::init(adapter).await {
							log::error!("failed to init adapter: {}", e);
						}
					}
				}
				Err(e) => log::error!("failed to discover adapters: {}", e),
			}
			debug!("adapters loaded");

			debug!("plugins loading...");
			match loader.plugins().await {
				Ok(plugins) => {
					for plugin in plugins {
						if let Err(e) = plugin::init(plugin).await {
							log::error!("failed to init plugin: {}", e);
						}
					}
				}
				Err(e) => log::error!("failed to discover plugins: {}", e),
			}
			debug!("plugins loaded");
		}

		if let Err(e) = EventEmitter::run() {
			log::error!("failed to start EventEmitter: {}", e);
		}

		info!("adapters: {}", puniyu_adapter_core::AdapterRegistry::all().len());
		info!("plugins: {}", puniyu_plugin_core::PluginRegistry::all().len());
		info!("commands: {}", puniyu_command::CommandRegistry::all().len());
		info!("handlers: {}", puniyu_handler::HandlerRegistry::all().len());

		info!(
			"{} initialized in {}",
			self.name.fg_rgb::<64, 224, 208>(),
			format_duration(start_time.elapsed()).fg_rgb::<255, 127, 80>()
		);

		#[cfg(feature = "server")]
		{
			self.routers.into_iter().for_each(puniyu_server::App::router);
			self.hoops.into_iter().for_each(puniyu_server::App::hoop);
			let config = puniyu_config::app::AppConfig::get().server();
            if let Some(logo) = self.assets {
                puniyu_server_router::logo::set_logo(logo);
            }
			tokio::spawn(puniyu_server::start_server(config.host(), config.port()));
		}
		tokio::signal::ctrl_c().await?;
		info!("Puniyu stopping...");
		puniyu_server::stop_server().await?;
		debug!("Server stopped");
		if let Some(callback) = self.on_exit {
			(callback)().await;
		}
		let uptime = start_time.elapsed();
		info!(
			"{} uptime: {}",
			self.name.to_case(Case::Lower).fg_rgb::<64, 224, 208>(),
			format_duration(uptime).fg_rgb::<255, 127, 80>()
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
async fn init_dir() -> io::Result<()> {
	let dirs = vec![
		puniyu_path::app_dir(),
		puniyu_path::adapter_dir(),
		puniyu_path::data_dir(),
		puniyu_path::config_dir(),
		puniyu_path::resource_dir(),
		puniyu_path::plugin_dir(),
		puniyu_path::log_dir(),
		puniyu_path::temp_dir(),
		puniyu_path::plugin::config_dir(),
		puniyu_path::plugin::data_dir(),
		puniyu_path::plugin::resource_dir(),
		puniyu_path::plugin::temp_dir(),
		puniyu_path::adapter::config_dir(),
		puniyu_path::adapter::data_dir(),
		puniyu_path::adapter::resource_dir(),
		puniyu_path::adapter::temp_dir(),
	];
	for dir in dirs {
		tokio::fs::create_dir_all(dir).await?;
	}
	Ok(())
}

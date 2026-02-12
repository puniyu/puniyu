mod adapter;
mod config;
mod hook;
mod loader;
mod plugin;
#[cfg(feature = "server")]
mod server;

use crate::{
	common, logger::log_init,
	logger::{debug, error, info, OwoColorize},
	VERSION,
};
use bytes::Bytes;
use convert_case::{Case, Casing};
use derive_builder::Builder;
use figlet_rs::FIGfont;
use puniyu_adapter::Adapter;
use puniyu_common::APP_NAME;
use puniyu_hook::types::{HookType, StatusType};
use puniyu_loader::{Loader};
use puniyu_path::{RESOURCE_DIR, WORKING_DIR};
use puniyu_plugin::Plugin;
use std::path::{PathBuf};
use std::sync::Arc;
use std::{env, io};
use tokio::{fs, signal};

#[derive(Builder)]
#[builder(setter(into), pattern = "mutable")]
pub struct App {
	#[builder(setter(name = "with_app_name"), default = "Self::default_app_name()")]
	app_name: String,
	#[builder(setter(name = "with_app_logo"),default)]
	app_logo: Option<Bytes>,
	#[builder(setter(name = "with_working_dir"),default = "Self::default_working_dir()")]
	working_dir: PathBuf,
	#[builder(setter(name = "with_plugin", custom), default)]
	plugins: Vec<Arc<dyn Plugin>>,
	#[builder(setter(name = "with_adapter", custom), default)]
	adapters: Vec<Arc<dyn Adapter>>,
	#[builder(setter(name = "with_loader", custom), default)]
	loaders: Vec<Arc<dyn Loader>>,
}

impl AppBuilder {
	pub(crate) fn default_app_name() -> String {
		String::from("puniyu")
	}
	pub(crate) fn default_working_dir() -> PathBuf {
		PathBuf::from(".")
	}

	pub fn with_plugin(&mut self, plugin: Arc<dyn Plugin>) -> &mut Self {
		if self.plugins.is_none() {
			self.plugins = Some(Vec::new());
		}
		self.plugins.as_mut().unwrap().push(plugin);
		self
	}
	pub fn with_adapter(&mut self, adapter: Arc<dyn Adapter>) -> &mut Self {
		if self.adapters.is_none() {
			self.adapters = Some(Vec::new());
		}
		self.adapters.as_mut().unwrap().push(adapter);
		self
	}
	pub fn with_loader(&mut self, loader: Arc<dyn Loader>) -> &mut Self {
		if self.loaders.is_none() {
			self.loaders = Some(Vec::new());
		}
		self.loaders.as_mut().unwrap().push(loader);
		self
	}
}

impl App {
	pub fn builder() -> AppBuilder {
		AppBuilder::default()
	}
	pub async fn run(self) -> io::Result<()> {
		use crate::common::time::format_duration;
		use puniyu_loader::LoaderRegistry;
		WORKING_DIR.get_or_init(|| self.working_dir);
		APP_NAME.get_or_init(|| self.app_name);
		use std::time::Duration;
		print_start_log();
		config::init_config();
		#[cfg(feature = "logger")]
		{
			log_init();
		}

		for loader in self.loaders.into_iter() {
			if let Err(e) = LoaderRegistry::register(loader) {
				error!("Failed to register loader: {}", e);
			}
		}
		let start_time = std::time::Instant::now();
		if let Err(e) = init_app(self.plugins, self.adapters, LoaderRegistry::all()).await {
			error!("Failed to init app: {}", e);
		}
		info!("hooks: {}", puniyu_hook::HookRegistry::all().len());
		let duration_str = format_duration(start_time.elapsed());
		execute_hooks(StatusType::Start).await;
		let app_name = APP_NAME.get().unwrap();
		info!(
			"{} 初始化完成，耗时: {}",
			app_name.to_case(Case::Lower).fg_rgb::<64, 224, 208>(),
			duration_str.fg_rgb::<255, 127, 80>()
		);
		let event_bus = puniyu_dispatch::EventBus::new();
		puniyu_dispatch::setup_event_bus(event_bus);
		let event_bus = puniyu_dispatch::get_event_bus();
		event_bus.run();
		#[cfg(feature = "server")]
		{
			use puniyu_config::Config;
			if let Some(logo) = self.app_logo.clone() {
				let logo_path = RESOURCE_DIR.join("logo.png");
				fs::write(&logo_path, &logo).await.expect("Failed to write");
				puniyu_server::set_logo(logo)
			}

			let config = Config::app();
			let config = config.server();
			let host = config.host();
			let port = config.port();
			puniyu_server::run_server_spawn(host, port);
		}

		signal::ctrl_c().await?;
		debug!("接收到中断信号，正在关闭...");
		execute_hooks(StatusType::Stop).await;
		info!(
			"{} 本次运行时间: {}",
			app_name.to_case(Case::Lower).fg_rgb::<64, 224, 208>(),
			format_duration(Duration::from_secs(common::time::uptime())).fg_rgb::<255, 127, 80>()
		);
		Ok(())
	}
}

async fn init_app(
	plugins: Vec<Arc<dyn Plugin>>,
	adapters: Vec<Arc<dyn Adapter>>,
	loaders: Vec<Arc<dyn Loader>>,
) -> io::Result<()> {
	for plugin in plugins {
		if let Err(e) = plugin::init_plugin(plugin).await {
			error!("Failed to register plugin: {}", e);
		}
	}
	for loader in loaders {
		if let Err(e) = loader::init_loader(loader).await {
			error!("Failed to register loader: {}", e);
		}
	}
	for adapter in adapters {
		if let Err(e) = adapter::init_adapter(adapter).await {
			error!("Failed to register adapter: {}", e);
		}
	}

	Ok(())
}

fn print_start_log() {
	let app_name = APP_NAME.get().unwrap();
	let app_name = app_name.to_case(Case::Pascal);
	if let Ok(standard_font) = FIGfont::standard()
		&& let Some(art_text) = standard_font.convert(app_name.as_str())
	{
		println!("{}", art_text);
	} else {
		println!("{}", app_name);
	}

	println!("{} 启动中...", app_name.to_case(Case::Lower));
	println!("版本: {}", VERSION);
	println!("Github: {}", env!("CARGO_PKG_REPOSITORY"));
}

async fn execute_hooks(status_type: StatusType) {
	use puniyu_hook::HookRegistry;
	let mut hooks = HookRegistry::all()
		.into_iter()
		.filter(|x| match x.builder.r#type() {
			HookType::Status(status) => status == &status_type,
			_ => false,
		})
		.collect::<Vec<_>>();
	hooks.sort_unstable_by_key(|a| a.builder.rank());

	for hook in hooks {
		if let Err(e) = hook.builder.run(None).await {
			match status_type {
				StatusType::Start => error!("启动hook钩子执行失败: {}", e),
				StatusType::Stop => error!("关闭hook钩子执行失败: {}", e),
			}
		}
		if let Err(e) = HookRegistry::unregister(hook.source) {
			error!("Failed to unregister hook: {}", e);
		}
	}
}

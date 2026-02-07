use crate::{
	VERSION, common,
	logger::log_init,
	logger::{OwoColorize, debug, error, info},
};
use bytes::Bytes;
use convert_case::{Case, Casing};
use figlet_rs::FIGfont;
pub use puniyu_common::APP_NAME;
use puniyu_common::path::{DATA_DIR, PLUGIN_DATA_DIR, PLUGIN_DIR, RESOURCE_DIR, WORKING_DIR};
use puniyu_config::{init_config, start_config_watcher};
use puniyu_event::init_event_bus;
use puniyu_registry::plugin::PluginType;
use puniyu_registry::{HookRegistry, adapter::AdapterRegistry, plugin::PluginRegistry};
use puniyu_types::adapter::Adapter;
use puniyu_types::hook::{HookType, StatusType};
use puniyu_types::plugin::Plugin;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::{env, env::consts::DLL_EXTENSION, io};
use tokio::{fs, signal};

pub struct AppBuilder {
	app_name: String,
	app_logo: Bytes,
	working_dir: PathBuf,
	plugins: Vec<&'static dyn Adapter>,
	adapters: Vec<&'static dyn Adapter>,
}

impl Default for AppBuilder {
	fn default() -> Self {
		Self {
			app_name: String::from("puniyu"),
			app_logo: Bytes::from_static(include_bytes!(concat!(
				env!("CARGO_MANIFEST_DIR"),
				"/assets/logo.png"
			))),
			working_dir: current_dir().unwrap(),
			plugins: Vec::new(),
			adapters: Vec::new(),
		}
	}
}

impl AppBuilder {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_name(mut self, name: impl Into<String>) -> Self {
		self.app_name = name.into();
		self
	}

	pub fn with_logo(mut self, logo: impl Into<Bytes>) -> Self {
		self.app_logo = logo.into();
		self
	}

	pub fn with_working_dir(mut self, path: &Path) -> Self {
		self.working_dir = path.to_path_buf();
		self
	}

	pub fn with_plugin(mut self, plugin: &'static dyn Adapter) -> Self {
		self.plugins.push(plugin);
		self
	}

	pub fn with_adapter(mut self, adapter: &'static dyn Adapter) -> Self {
		self.adapters.push(adapter);
		self
	}

	pub fn build(self) -> App {
		WORKING_DIR.get_or_init(|| self.working_dir.clone());
		APP_NAME.get_or_init(|| self.app_name.clone());
		#[cfg(feature = "server")]
		{
			puniyu_server::LOGO.get_or_init(|| self.app_logo.clone());
		}
		App { builder: self }
	}
}

pub struct App {
	builder: AppBuilder,
}

impl App {
	pub fn builder() -> AppBuilder {
		AppBuilder::new()
	}
	pub async fn run(&self) -> io::Result<()> {
		use crate::common::format_duration;
		use std::time::Duration;
		print_start_log();
		init_config();
		#[cfg(feature = "logger")]
		{
			log_init();
		}
		let start_time = std::time::Instant::now();
		let app_name = APP_NAME.get().unwrap();
		init_app(&self.builder.plugins, &self.builder.adapters).await;
		info!("钩子数量: {}", HookRegistry::all().len());
		start_config_watcher();
		let duration_str = format_duration(start_time.elapsed());
		execute_hooks(StatusType::Start).await;
		info!(
			"{} 初始化完成，耗时: {}",
			app_name.to_case(Case::Lower).fg_rgb::<64, 224, 208>(),
			duration_str.fg_rgb::<255, 127, 80>()
		);

		init_event_bus();
		#[cfg(feature = "server")]
		{
			use crate::config::Config;
			use std::net::IpAddr;
			let logo_path = RESOURCE_DIR.join("logo.png");
			if !logo_path.exists() {
				fs::write(&logo_path, &self.builder.app_logo).await.expect("写入logo失败");
			}
			let config = Config::app();
			let config = config.server();
			let host = IpAddr::V4(config.host().parse().unwrap());
			let port = config.port();
			puniyu_server::run_server_spawn(Some(host), Some(port));
		}

		signal::ctrl_c().await?;
		debug!("接收到中断信号，正在关闭...");
		execute_hooks(StatusType::Stop).await;
		info!(
			"{} 本次运行时间: {}",
			app_name.to_case(Case::Lower).fg_rgb::<64, 224, 208>(),
			format_duration(Duration::from_secs(common::uptime())).fg_rgb::<255, 127, 80>()
		);
		Ok(())
	}
}

async fn init_app(
    plugins: &[&'static dyn Adapter],
    adapters: &[&'static dyn Adapter],
) {
	if !DATA_DIR.as_path().exists() {
		fs::create_dir(DATA_DIR.as_path()).await.unwrap();
	}

	if !RESOURCE_DIR.as_path().exists() {
		fs::create_dir(RESOURCE_DIR.as_path()).await.unwrap();
	}
	init_plugin(plugins).await;
	init_adapter(adapters).await;
}

async fn init_plugin(plugins: &[&'static dyn Adapter]) {
	if !PLUGIN_DIR.as_path().exists() {
		fs::create_dir(PLUGIN_DIR.as_path()).await.expect("Failed to create plugin directory");
	}

	if !PLUGIN_DATA_DIR.as_path().exists() {
		fs::create_dir(PLUGIN_DATA_DIR.as_path())
			.await
			.expect("Failed to create plugin data directory");
	}
	let mut plugins_list =
		plugins.iter().map(|p| PluginType::Builder(*p)).collect::<Vec<PluginType>>();

	let pattern = PLUGIN_DIR.join(format!("*.{}", DLL_EXTENSION));
	if let Ok(paths) = glob::glob(pattern.to_str().expect("Failed to parse plugin path")) {
		for entry in paths.filter_map(Result::ok) {
			plugins_list.push(entry.into());
		}
	}

	PluginRegistry::load_plugins(plugins_list).await.unwrap_or_else(|e| {
		error!("插件加载失败: {:?}", e);
	});

	let plugin_count = PluginRegistry::get_all_plugins().len();
	debug!(
		"{}: {} {}",
		"共加载".fg_rgb::<135, 206, 250>(),
		plugin_count,
		"个插件".fg_rgb::<135, 206, 250>()
	)
}

async fn init_adapter(adapters: &[&'static dyn Adapter]) {
	AdapterRegistry::load_adapters(adapters).await.unwrap_or_else(|e| {
		error!("适配器加载失败: {:?}", e);
	});

	let adapter_count = AdapterRegistry::adapters().len();

	debug!(
		"{}: {} {}",
		"共加载".fg_rgb::<135, 206, 250>(),
		adapter_count,
		"个适配器".fg_rgb::<135, 206, 250>()
	)
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
	let mut hooks = HookRegistry::all()
		.into_iter()
		.filter(|x| match x.builder.r#type() {
			HookType::Status(status) => status == status_type,
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
		HookRegistry::unregister(hook.index);
	}
}

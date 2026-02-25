mod adapter;
mod config;
mod hook;
mod loader;
mod plugin;
#[cfg(feature = "server")]
mod server;

use crate::{
	VERSION, common,
	logger::log_init,
	logger::{OwoColorize, debug, error, info},
};
use bytes::Bytes;
use convert_case::{Case, Casing};
use figlet_rs::FIGfont;
use puniyu_adapter::Adapter;
use puniyu_common::APP_NAME;
use puniyu_handler::Handler;
use puniyu_hook::types::{HookType, StatusType};
use puniyu_loader::Loader;
use puniyu_path::{RESOURCE_DIR, WORKING_DIR};
use puniyu_plugin::Plugin;
use std::path::PathBuf;
use std::sync::Arc;
use std::{env, io};
use tokio::{fs, signal};

/// 应用构建器
///
/// 使用构建器模式配置和创建应用实例
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_core::App;
///
/// let app = App::builder()
///     .with_app_name("my_bot")
///     .with_plugin(MyPlugin)
///     .with_adapter(MyAdapter)
///     .build()
///     .unwrap();
///
/// app.run().await?;
/// ```
pub struct AppBuilder {
	app_name: String,
	app_logo: Option<Bytes>,
	working_dir: PathBuf,
	plugins: Vec<Arc<dyn Plugin>>,
	adapters: Vec<Arc<dyn Adapter>>,
	loaders: Vec<Arc<dyn Loader>>,
	handlers: Vec<Arc<dyn Handler>>,
}

impl Default for AppBuilder {
	fn default() -> Self {
		Self {
			app_name: String::from("puniyu"),
			app_logo: None,
			working_dir: PathBuf::from("."),
			plugins: Vec::new(),
			adapters: Vec::new(),
			loaders: Vec::new(),
			handlers: Vec::new(),
		}
	}
}

impl AppBuilder {
	/// 设置应用名称
	///
	/// # 参数
	///
	/// - `name`: 应用名称
	pub fn with_app_name(mut self, name: impl Into<String>) -> Self {
		self.app_name = name.into();
		self
	}

	/// 设置应用 Logo
	///
	/// # 参数
	///
	/// - `logo`: Logo 图片的字节数据
	pub fn with_app_logo(mut self, logo: Bytes) -> Self {
		self.app_logo = Some(logo);
		self
	}

	/// 设置工作目录
	///
	/// # 参数
	///
	/// - `dir`: 工作目录路径
	pub fn with_working_dir(mut self, dir: impl Into<PathBuf>) -> Self {
		self.working_dir = dir.into();
		self
	}

	/// 添加插件
	///
	/// 接受任何实现了 `Plugin` trait 的类型，在编译期确定
	///
	/// # 参数
	///
	/// - `plugin`: 插件实例
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// App::builder()
	///     .with_plugin(MyPlugin::new())
	///     .with_plugin(AnotherPlugin)
	/// ```
	pub fn with_plugin<P: Plugin + 'static>(mut self, plugin: P) -> Self {
		self.plugins.push(Arc::new(plugin));
		self
	}

	/// 添加适配器
	///
	/// 接受任何实现了 `Adapter` trait 的类型，在编译期确定
	///
	/// # 参数
	///
	/// - `adapter`: 适配器实例
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// App::builder()
	///     .with_adapter(ConsoleAdapter::new())
	///     .with_adapter(HttpAdapter::new())
	/// ```
	pub fn with_adapter<A: Adapter + 'static>(mut self, adapter: A) -> Self {
		self.adapters.push(Arc::new(adapter));
		self
	}

	/// 添加加载器
	///
	/// 接受任何实现了 `Loader` trait 的类型，在编译期确定
	///
	/// # 参数
	///
	/// - `loader`: 加载器实例
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// App::builder()
	///     .with_loader(PluginLoader::new())
	/// ```
	pub fn with_loader<L: Loader + 'static>(mut self, loader: L) -> Self {
		self.loaders.push(Arc::new(loader));
		self
	}

	/// 添加处理器
	///
	/// 接受任何实现了 `Handler` trait 的类型，在编译期确定
	///
	/// # 参数
	///
	/// - `handler`: 处理器实例
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// App::builder()
	///     .with_handler(MyHandler::new())
	///     .with_handler(AnotherHandler)
	/// ```
	pub fn with_handler<H: Handler + 'static>(mut self, handler: H) -> Self {
		self.handlers.push(Arc::new(handler));
		self
	}

	/// 构建应用实例
	///
	/// # 返回值
	///
	/// 返回配置好的 `App` 实例
	pub fn build(self) -> App {
		App {
			app_name: self.app_name,
			app_logo: self.app_logo,
			working_dir: self.working_dir,
			plugins: self.plugins,
			adapters: self.adapters,
			loaders: self.loaders,
			handlers: self.handlers,
		}
	}
}

pub struct App {
	app_name: String,
	app_logo: Option<Bytes>,
	working_dir: PathBuf,
	plugins: Vec<Arc<dyn Plugin>>,
	adapters: Vec<Arc<dyn Adapter>>,
	loaders: Vec<Arc<dyn Loader>>,
	handlers: Vec<Arc<dyn Handler>>,
}

impl App {
	pub fn builder() -> AppBuilder {
		AppBuilder::default()
	}
	/// 运行应用
	///
	/// 初始化所有组件并启动应用，直到接收到中断信号
	///
	/// # 返回值
	///
	/// 成功返回 `Ok(())`，失败返回 IO 错误
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let app = App::builder().build();
	/// app.run().await?;
	/// ```
	pub async fn run(self) -> io::Result<()> {
		use crate::common::time::format_duration;
		use puniyu_loader::LoaderRegistry;
		use std::time::Duration;

		WORKING_DIR.get_or_init(|| self.working_dir);
		APP_NAME.get_or_init(|| self.app_name.clone());

		print_start_log();
		config::init_config();

		#[cfg(feature = "logger")]
		{
			log_init();
		}
		for handler in self.handlers.into_iter() {
			if let Err(e) = puniyu_handler::HandlerRegistry::register(handler) {
				error!("Failed to register handler: {}", e);
			}
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

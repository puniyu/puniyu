use puniyu_context::{Context, PluginContext};
use puniyu_plugin_core::Plugin;
use puniyu_service::Service;
use smol_str::SmolStr;
use std::cmp::Reverse;
use std::path::PathBuf;
use std::sync::Arc;

struct PluginEntry {
	plugin: Arc<dyn Plugin>,
	ctx: PluginContext,
}

struct Inner {
	ctx: Arc<Context>,
	plugins: Vec<PluginEntry>,
}

pub struct Puniyu {
	inner: Inner,
}

impl Puniyu {
	pub fn new(name: impl Into<SmolStr>, cwd_dir: impl Into<PathBuf>) -> Self {
		let path = puniyu_path::Path::new(name, cwd_dir);
		Self { inner: Inner { ctx: Arc::new(Context::new(path)), plugins: Vec::new() } }
	}

	/// 注册全局服务。
	pub fn provide<S: Service>(&self, service: S) {
		let ctx = &self.inner.ctx;
		if let Err(e) = ctx.inject(service) {
			log::error!("inject failed: {e}");
		}
	}

	/// 加载插件，自动检查 using() 依赖并注入到插件作用域。
	pub fn load_plugin(&mut self, plugin: impl Plugin + 'static) {
		let plugin = Arc::new(plugin);
		let name = plugin.name();
		let deps = plugin.dependencies();

		for dep in &deps {
			if self.inner.ctx.require(dep).is_none() {
				log::error!("plugin '{name}' missing dependency: '{dep}'");
				return;
			}
		}

		let ctx = self.inner.ctx.plugin();
		for dep in &deps {
			if let Some(service) = self.inner.ctx.require(dep) {
				ctx.inject(service);
			}
		}

		log::info!("plugin loaded: {name}");
		self.inner.plugins.push(PluginEntry { plugin, ctx });
	}

	/// 启动应用。
	pub async fn run(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let start_time = std::time::Instant::now();

		// 按 priority 排序
		let mut sorted: Vec<_> = self.inner.plugins.iter().collect();
		sorted.sort_by_key(|e| e.plugin.priority());

		// on_start
		for entry in &sorted {
			if let Err(e) = entry.plugin.on_start(&entry.ctx).await {
				log::error!("plugin '{}' start failed: {e}", entry.plugin.name());
			}
		}

		// on_load
		for entry in &sorted {
			if let Err(e) = entry.plugin.on_load(&entry.ctx).await {
				log::error!("plugin '{}' load failed: {e}", entry.plugin.name());
			}
		}

		log::info!("puniyu initialized in {:.2?}", start_time.elapsed());

		// 等待退出信号
		tokio::signal::ctrl_c().await?;
		log::info!("shutting down...");

		let mut rev: Vec<_> = self.inner.plugins.iter().collect();
		rev.sort_by_key(|e| Reverse(e.plugin.priority()));

		for entry in &rev {
			if let Err(e) = entry.plugin.on_unload(&entry.ctx).await {
				log::error!("plugin '{}' unload failed: {e}", entry.plugin.name());
			}
		}

		for entry in &rev {
			if let Err(e) = entry.plugin.on_stop(&entry.ctx).await {
				log::error!("plugin '{}' stop failed: {e}", entry.plugin.name());
			}
		}

		log::info!("puniyu uptime: {:.2?}", start_time.elapsed());
		Ok(())
	}
}

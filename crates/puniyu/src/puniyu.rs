use puniyu_context::{Context, SubContext};
use puniyu_plugin::Plugin;
use smol_str::SmolStr;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

struct PluginEntry {
	plugin: Arc<dyn Plugin>,
	ctx: SubContext,
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

	pub fn load_plugin(&mut self, plugin: impl Plugin + 'static) {
		let plugin = Arc::new(plugin);
		let name = plugin.name();
		let deps = plugin.dependencies();

		let ctx = self.inner.ctx.sub();

		log::info!("plugin loaded: {name}");
		self.inner.plugins.push(PluginEntry { plugin, ctx });
	}

	pub async fn run(self) -> io::Result<()> {
		for entry in &self.inner.plugins {
			if let Err(e) = entry.plugin.on_start(&entry.ctx).await {
				log::error!("plugin {} on_start failed: {e}", entry.plugin.name());
			}
		}

		for entry in &self.inner.plugins {
			if let Err(e) = entry.plugin.on_load(&entry.ctx).await {
				log::error!("plugin {} on_load failed: {e}", entry.plugin.name());
			}
		}

		
		tokio::signal::ctrl_c().await?;

		log::info!("shutting down...");


		for entry in self.inner.plugins.iter().rev() {
			if let Err(e) = entry.plugin.on_unload(&entry.ctx).await {
				log::error!("plugin {} on_unload failed: {e}", entry.plugin.name());
			}
			if let Err(e) = entry.plugin.on_stop(&entry.ctx).await {
				log::error!("plugin {} on_stop failed: {e}", entry.plugin.name());
			}
		}

		Ok(())
	}
}

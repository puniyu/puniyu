use puniyu_context::{Context, SubContext};
use puniyu_plugin_core::Plugin;
use puniyu_service::Service;
use smol_str::SmolStr;
use std::cmp::Reverse;
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

	/// 加载插件，自动检查 using() 依赖并注入到插件作用域。
	pub fn load_plugin(&mut self, plugin: impl Plugin + 'static) {
		let plugin = Arc::new(plugin);
		let name = plugin.name();
		let deps = plugin.dependencies();

		let ctx = self.inner.ctx.sub();

		log::info!("plugin loaded: {name}");
		self.inner.plugins.push(PluginEntry { plugin, ctx });
	}

	/// 启动应用。
	pub async fn run(self) -> io::Result<()> {
		Ok(())
	}
}

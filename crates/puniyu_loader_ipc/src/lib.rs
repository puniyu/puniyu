mod plugin;
mod registry;

use std::sync::Arc;

use puniyu_plugin_core::Plugin;

pub struct Loader;

impl puniyu_loader::Loader for Loader {
	fn name(&self) -> &'static str {
		env!("CARGO_PKG_NAME")
	}

	fn plugins(&self) -> Vec<Arc<dyn Plugin>> {
		Vec::new()
	}
}

use std::sync::Arc;

use puniyu_loader::Loader;
use puniyu_plugin_core::Plugin;

pub struct IpcLoader;

impl Loader for IpcLoader {
	fn name(&self) -> &'static str {
		env!("CARGO_PKG_NAME")
	}
	fn plugins(&self) -> Vec<Arc<dyn Plugin>> {
		vec![]
	}
}

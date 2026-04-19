mod plugin;
mod registry;
#[doc(inline)]
pub use registry::IpcRegistry;
mod types;
#[doc(inline)]
pub use types::*;
mod cilent;
mod process;
pub mod service;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use log::{debug, warn};
use puniyu_plugin_core::Plugin;
use walkdir::WalkDir;

pub struct Loader;

#[async_trait]
impl puniyu_loader::Loader for Loader {
	fn name(&self) -> &'static str {
		env!("CARGO_PKG_NAME")
	}

	async fn plugins(&self) -> Vec<Arc<dyn Plugin>> {
		if let Err(error) = load_plugins().await {
			warn!("failed to load ipc plugins: {error}");
		}
		Vec::new()
	}
}

async fn load_plugins() -> std::io::Result<()> {
	for path in find_plugin_executables() {
		let Ok(process) = IpcProcess::from_path(&path) else {
			continue;
		};

		match plugin::probe_plugin(process).await {
			Ok(process) => {
				if let Err(error) = registry::IpcRegistry::register(process) {
					warn!("failed to register plugin {}: {error}", path.display());
				}
			}
			Err(error) => {
				warn!("failed to probe plugin {}: {error}", path.display());
			}
		}
	}
	Ok(())
}

fn find_plugin_executables() -> Vec<PathBuf> {
	WalkDir::new(puniyu_path::plugin_dir())
		.max_depth(2)
		.sort_by(|a, b| a.file_name().cmp(b.file_name()))
		.into_iter()
		.filter_map(|e| {
			let entry = e.map_err(std::io::Error::other)
				.inspect_err(|e| warn!("failed to read dir entry: {e}"))
				.ok()?;
			is_executable_candidate(entry.path()).then(|| entry.into_path())
		})
		.collect()
}

fn is_executable_candidate(path: &Path) -> bool {
	if !path.is_file() {
		debug!("skip non-file path: {}", path.display());
		return false;
	}

	#[cfg(windows)]
	let result =
		path.extension().and_then(OsStr::to_str).is_some_and(|ext| ext.eq_ignore_ascii_case("exe"));

	#[cfg(unix)]
	let result = {
		use std::os::unix::fs::PermissionsExt;
		std::fs::metadata(path).map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or_else(|_| {
			debug!("failed to read metadata for path: {}", path.display());
			false
		})
	};

	debug!("check executable candidate: {} => {}", path.display(), result);
	result
}

mod plugin;
mod registry;
#[doc(inline)]
pub use registry::IpcPluginRegistry;
mod types;
#[doc(inline)]
pub use types::*;
mod processes;
mod protocol;

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
		if let Err(error) = bootstrap_processes().await {
			warn!("failed to bootstrap ipc plugin processes: {error}");
		}
		Vec::new()
	}
}

async fn bootstrap_processes() -> std::io::Result<()> {
	for path in get_plugin_paths()? {
		let Ok(process) = IpcProcess::from_path(&path) else {
			continue;
		};

		match processes::probe_process(process).await {
			Ok(process) => {
				if let Err(error) = registry::IpcPluginRegistry::register(process) {
					warn!("failed to register ipc plugin process {}: {error}", path.display());
				}
			}
			Err(error) => {
				warn!("failed to probe ipc plugin {}: {error}", path.display());
			}
		}
	}
	Ok(())
}

fn get_plugin_paths() -> std::io::Result<Vec<PathBuf>> {
	let mut paths = Vec::new();
	for entry in WalkDir::new(puniyu_path::plugin_dir()) {
		let entry = entry.map_err(std::io::Error::other)?;
		let path = entry.path();
		if is_executable_candidate(path) {
			paths.push(path.to_path_buf());
		}
	}
	Ok(paths)
}

fn is_executable_candidate(path: &Path) -> bool {
	if !path.is_file() {
		debug!("skip non-file path: {}", path.display());
		return false;
	}

	if cfg!(windows) {
		let is_exe = path
			.extension()
			.and_then(OsStr::to_str)
			.is_some_and(|ext| ext.eq_ignore_ascii_case("exe"));
		debug!("check executable candidate (windows): {} => {}", path.display(), is_exe);
		return is_exe;
	}

	#[cfg(unix)]
	{
		use std::os::unix::fs::PermissionsExt;
		if let Ok(metadata) = std::fs::metadata(path) {
			let is_executable = metadata.permissions().mode() & 0o111 != 0;
			debug!("check executable candidate (unix): {} => {}", path.display(), is_executable);
			return is_executable;
		}
		debug!("failed to read metadata for path: {}", path.display());
		return false;
	}

	#[cfg(not(unix))]
	{
		debug!("check executable candidate (fallback): {} => true", path.display());
		true
	}
}

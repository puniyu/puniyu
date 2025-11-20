mod error;
pub use error::Error;
pub use libloading;

use libloading::Library;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct LibraryInfo {
	pub name: String,
	pub path: PathBuf,
	pub library: Arc<Library>,
}

impl From<PathBuf> for LibraryInfo {
	fn from(path: PathBuf) -> Self {
		let lib = unsafe { Library::new(&path).unwrap() };
		let name = path.file_name().unwrap().to_string_lossy().to_string();
		Self { name, path, library: Arc::new(lib) }
	}
}

#[derive(Default)]
struct LibraryStore(HashMap<String, Arc<LibraryInfo>>);
impl LibraryStore {
	pub fn insert(&mut self, name: String, lib: Arc<LibraryInfo>) {
		if self.0.contains_key(&name) {
			return;
		}
		self.0.insert(name, lib);
	}
}

#[derive(Default)]
pub struct LibraryRegistry {
	store: LibraryStore,
}

impl LibraryRegistry {
	pub fn load_library(&mut self, path: &Path) -> Result<(), Error> {
		let name = path
			.file_name()
			.map(|n| n.to_string_lossy().to_string())
			.ok_or_else(|| Error::NotFound(path.to_string_lossy().to_string()))?;
		if self.store.0.contains_key(&name) {
			return Ok(());
		}

		let lib_path = path.to_path_buf();
		let library_info: LibraryInfo = lib_path.into();
		self.store.insert(name.to_string(), library_info.into());

		Ok(())
	}

	pub fn get_library(&self, name: &str) -> Option<Arc<LibraryInfo>> {
		self.store.0.get(name).cloned()
	}
	pub fn remove_library(&mut self, name: &str) {
		let _ = self.store.0.remove(name);
	}

	pub fn reload_library(&mut self, name: &str) -> Result<(), Error> {
		if !self.store.0.contains_key(name) {
			return Err(Error::NotFound(name.to_string()));
		}
		if let Some(old_lib) = self.store.0.remove(name) {
			drop(old_lib);
		}
		let lib_path = self.get_library(name).unwrap().path.clone();
		let library_info: LibraryInfo = lib_path.into();
		self.store.insert(name.to_string(), library_info.into());
		Ok(())
	}
}

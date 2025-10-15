use crate::error::Library as Error;
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
pub struct LibraryStore(HashMap<String, Arc<LibraryInfo>>);
impl LibraryStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert_library(&mut self, name: String, lib: Arc<LibraryInfo>) {
		if self.0.contains_key(&name) {
			return;
		}
		self.0.insert(name, lib);
	}
	pub fn get_library(&self, name: &str) -> Option<Arc<LibraryInfo>> {
		self.0.get(name).cloned()
	}
	pub fn remove_library(&mut self, name: &str) -> bool {
		self.0.remove(name).is_some()
	}
}
#[derive(Default)]
pub struct PluginLibrary {
	store: LibraryStore,
}

impl PluginLibrary {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn load_plugin(&mut self, path: &Path) -> Result<&mut Self, Error> {
		let name = path
			.file_name()
			.map(|n| n.to_string_lossy().to_string())
			.ok_or_else(|| Error::NotFound(path.to_string_lossy().to_string()))?;
		if self.store.0.contains_key(&name) {
			return Ok(self);
		}

		let lib_path = path.to_path_buf();
		let library_info: LibraryInfo = lib_path.into();
		self.store.insert_library(name.to_string(), library_info.into());

		Ok(self)
	}

	/// 从内部存储中获取已加载的库
	pub fn get_plugin(&self, name: &str) -> Option<Arc<LibraryInfo>> {
		self.store.0.get(name).cloned()
	}

	/// 移除已加载的库
	pub fn remove_plugin(&mut self, name: &str) -> bool {
		self.store.0.remove(name).is_some()
	}

	/// 重新加载已加载的库
	pub fn reload_plugin(&mut self, name: &str) -> Result<(), Error> {
		if !self.store.0.contains_key(name) {
			return Err(Error::NotFound(name.to_string()));
		}
		if let Some(old_lib) = self.store.0.remove(name) {
			drop(old_lib);
		}
		let lib_path = self.get_plugin(name).unwrap().path.clone();
		let library_info: LibraryInfo = lib_path.into();
		self.store.insert_library(name.to_string(), library_info.into());
		Ok(())
	}
}

#[derive(Default)]
pub struct AdapterLibrary {
	store: LibraryStore,
}

impl AdapterLibrary {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn load_adapter(&mut self, path: &Path) -> Result<&mut Self, Error> {
		let name = path
			.file_name()
			.map(|n| n.to_string_lossy().to_string())
			.ok_or_else(|| Error::NotFound(path.to_string_lossy().to_string()))?;
		if self.store.0.contains_key(&name) {
			return Ok(self);
		}

		let lib_path = path.to_path_buf();

		let library_info: LibraryInfo = lib_path.into();
		self.store.insert_library(name.to_string(), library_info.into());
		Ok(self)
	}

	pub fn get_adapter(&self, name: &str) -> Option<Arc<LibraryInfo>> {
		self.store.0.get(name).cloned()
	}

	pub fn remove_adapter(&mut self, name: &str) -> bool {
		self.store.0.remove(name).is_some()
	}

	pub fn reload(&mut self, name: &str) -> Result<(), Error> {
		if !self.store.0.contains_key(name) {
			return Err(Error::NotFound(name.to_string()));
		}
		if let Some(old_lib) = self.store.0.remove(name) {
			drop(old_lib);
		}
		let lib_path = self.get_adapter(name).unwrap().path.clone();
		let library_info: LibraryInfo = lib_path.into();
		self.store.insert_library(name.to_string(), library_info.into());
		Ok(())
	}
}

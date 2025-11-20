mod error;
pub use error::Error;
pub use libloading;

use libloading::Library;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex};

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

static LIBRARY_STORE: LazyLock<Mutex<HashMap<String, Arc<LibraryInfo>>>> = 
	LazyLock::new(|| Mutex::new(HashMap::new()));

pub struct LibraryRegistry;

impl LibraryRegistry {
	pub fn load_library(path: &Path) -> Result<(), Error> {
		let name = path
			.file_name()
			.map(|n| n.to_string_lossy().to_string())
			.ok_or_else(|| Error::NotFound(path.to_string_lossy().to_string()))?;
		
		let mut store = LIBRARY_STORE.lock().unwrap();
		if store.contains_key(&name) {
			return Ok(());
		}

		let lib_path = path.to_path_buf();
		let library_info: LibraryInfo = lib_path.into();
		store.insert(name, Arc::new(library_info));

		Ok(())
	}

	pub fn get_library(name: &str) -> Option<Arc<LibraryInfo>> {
		let store = LIBRARY_STORE.lock().unwrap();
		store.get(name).cloned()
	}

	pub fn remove_library(name: &str) {
		let mut store = LIBRARY_STORE.lock().unwrap();
		let _ = store.remove(name);
	}

	pub fn reload_library(name: &str) -> Result<(), Error> {
		let mut store = LIBRARY_STORE.lock().unwrap();
		
		if !store.contains_key(name) {
			return Err(Error::NotFound(name.to_string()));
		}
		
		let lib_path = store.get(name).unwrap().path.clone();
		store.remove(name);
		
		let library_info: LibraryInfo = lib_path.into();
		store.insert(name.to_string(), Arc::new(library_info));
		
		Ok(())
	}
}

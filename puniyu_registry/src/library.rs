use crate::error::Library as Error;
use crate::library;
use hashbrown::HashMap;
use libloading::Library;
use std::{env::consts::DLL_EXTENSION, path::PathBuf, sync::Arc};

pub struct LibraryStore {
    libs: HashMap<String, Arc<Library>>,
}
impl LibraryStore {
    pub fn new() -> Self {
        Self {
            libs: HashMap::new(),
        }
    }

    pub fn insert_library(&mut self, name: String, lib: Arc<Library>) {
        if self.libs.contains_key(&name) {
            return;
        }
        self.libs.insert(name, lib);
    }

    pub fn get_library(&self, name: &str) -> Option<Arc<Library>> {
        self.libs.get(name).cloned()
    }

    pub fn remove_library(&mut self, name: &str) -> bool {
        self.libs.remove(name).is_some()
    }
}
impl Default for LibraryStore {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct PluginLibrary {
    store: LibraryStore,
}

impl PluginLibrary {
    /// 创建一个新的 PluginLibrary 实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 加载插件库并存储在内部 HashMap 中
    /// 如果库已加载则不进行加载
    ///
    /// # Safety
    ///
    /// 调用者必须确保传入的库路径是有效的，并且库文件存在且可加载。
    /// 调用者还必须确保在使用从库中获取的符号时遵循正确的安全实践。
    pub fn load_plugin(&mut self, name: &str) -> Result<&mut Self, Error> {
        if self.store.libs.contains_key(name) {
            return Ok(self);
        }

        let lib_path = PathBuf::from(format!("plugins/{}.{}", name, DLL_EXTENSION));
        let lib = Arc::new(unsafe {
            Library::new(lib_path).map_err(|_| Error::NotFound(name.to_string()))?
        });
        self.store.insert_library(name.to_string(), lib);

        Ok(self)
    }

    /// 从内部存储中获取已加载的库
    pub fn get_plugin(&self, name: &str) -> Option<&Arc<Library>> {
        self.store.libs.get(name)
    }

    /// 移除已加载的库
    pub fn remove_plugin(&mut self, name: &str) -> bool {
        self.store.libs.remove(name).is_some()
    }

    /// 重新加载已加载的库
    pub fn reload_plugin(&mut self, name: &str) -> Result<(), Error> {
        self.remove_plugin(name);
        let lib_path = PathBuf::from(format!("plugins/{}.{}", name, DLL_EXTENSION));
        let lib = Arc::new(unsafe {
            Library::new(lib_path).map_err(|_| Error::NotFound(name.to_string()))?
        });
        self.store.insert_library(name.to_string(), lib);
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
    pub fn load_adapter(&mut self, name: &str) -> Result<&mut Self, Error> {
        if self.store.libs.contains_key(name) {
            return Ok(self);
        }

        let lib_path = PathBuf::from(format!("adapters/{}.{}", name, DLL_EXTENSION));
        let lib = Arc::new(unsafe {
            Library::new(lib_path).map_err(|_| Error::NotFound(name.to_string()))?
        });
        self.store.insert_library(name.to_string(), lib);
        Ok(self)
    }

    pub fn get(&self, name: &str) -> Option<&Arc<Library>> {
        self.store.libs.get(name)
    }

    pub fn remove_plugin(&mut self, name: &str) -> bool {
        self.store.libs.remove(name).is_some()
    }

    pub fn reload(&mut self, name: &str) -> Result<(), Error> {
        self.remove_plugin(name);
        let lib_path = PathBuf::from(format!("adapters/{}.{}", name, DLL_EXTENSION));
        let lib = Arc::new(unsafe {
            Library::new(lib_path).map_err(|_| Error::NotFound(name.to_string()))?
        });
        self.store.insert_library(name.to_string(), lib);
        Ok(())
    }
}

use hashbrown::HashMap;
use libloading::Library;
use std::env::consts::DLL_EXTENSION;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct PluginLibrary {
    lib: HashMap<String, Arc<Library>>,
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
    pub fn load_library(&mut self, name: &str) -> Result<&mut Self, Box<dyn std::error::Error>> {
        if self.lib.contains_key(name) {
            return Ok(self);
        }

        let lib_path = PathBuf::from(format!("plugins/{}.{}", name, DLL_EXTENSION));
        let lib = Arc::new(unsafe { Library::new(lib_path)? });
        self.lib.insert(name.to_string(), lib);
        Ok(self)
    }

    /// 从内部存储中获取已加载的库
    pub fn get(&self, name: &str) -> Option<&Arc<Library>> {
        self.lib.get(name)
    }

    /// 移除已加载的库
    pub fn remove(&mut self, name: &str) -> bool {
        self.lib.remove(name).is_some()
    }

    /// 重新加载已加载的库
    pub fn reload(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.remove(name);
        let lib_path = PathBuf::from(format!("plugins/{}.{}", name, DLL_EXTENSION));
        let lib = Arc::new(unsafe { Library::new(lib_path)? });
        self.lib.insert(name.to_string(), lib);
        Ok(())
    }
}

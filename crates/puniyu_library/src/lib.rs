//! # puniyu_library
//!
//! 动态库管理模块，提供动态库的加载、卸载和管理功能。
//!
//! ## 概述
//!
//! `puniyu_library` 基于 `libloading` 实现动态库的加载、获取、卸载和重载功能，
//! 使用全局注册表统一管理所有已加载的动态库。
//!
//! ## 特性
//!
//! - 🎯 **类型安全** - 使用 Rust 类型系统确保动态库操作的安全性
//! - 🔧 **全局管理** - 提供全局注册表统一管理所有动态库
//! - 🔄 **热重载** - 支持动态库的热重载功能
//! - 📦 **线程安全** - 使用 `Mutex` 保证多线程环境下的安全性
//! - 🎨 **易于使用** - 提供简洁的 API 接口
//!
//! ## 使用方式
//!
//! ### 加载动态库
//!
//! ```rust,ignore
//! use puniyu_library::LibraryRegistry;
//! use std::path::Path;
//!
//! // 加载动态库
//! LibraryRegistry::load(Path::new("plugin.dll"))?;
//!
//! // 或使用字符串路径
//! LibraryRegistry::load_from_path("plugin.dll")?;
//! ```
//!
//! ### 获取和使用动态库
//!
//! ```rust,ignore
//! use puniyu_library::LibraryRegistry;
//!
//! // 获取库信息
//! if let Some(lib) = LibraryRegistry::get("plugin.dll") {
//!     // 使用库
//!     unsafe {
//!         let func: libloading::Symbol<fn() -> i32> = lib.library.get(b"my_function")?;
//!         let result = func();
//!     }
//! }
//! ```
//!
//! ### 卸载和重载
//!
//! ```rust,ignore
//! use puniyu_library::LibraryRegistry;
//!
//! // 卸载库
//! LibraryRegistry::unload("plugin.dll")?;
//!
//! // 重新加载库
//! LibraryRegistry::reload("plugin.dll")?;
//! ```

mod error;
pub use error::Error;
pub use libloading;

use libloading::Library;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex};

/// 动态库信息
///
/// 包含动态库的名称、路径和库句柄。
///
/// # 字段
///
/// - `name` - 库名称（从文件名提取）
/// - `path` - 库文件的完整路径
/// - `library` - 动态库句柄的 Arc 包装
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_library::LibraryInfo;
/// use std::path::PathBuf;
///
/// let info = LibraryInfo::new(PathBuf::from("plugin.dll"))?;
/// println!("库名称: {}", info.name);
/// println!("库路径: {:?}", info.path);
/// ```
#[derive(Debug, Clone)]
pub struct LibraryInfo {
	/// 库名称
	pub name: String,
	/// 库文件路径
	pub path: PathBuf,
	/// 动态库句柄
	pub library: Arc<Library>,
}

impl LibraryInfo {
	/// 创建新的库信息
	///
	/// # 参数
	///
	/// - `path` - 库文件路径
	///
	/// # 返回
	///
	/// - `Ok(LibraryInfo)` - 成功创建库信息
	/// - `Err(Error)` - 创建失败
	///
	/// # 错误
	///
	/// - `Error::InvalidPath` - 路径无效或无法提取文件名
	/// - `Error::LoadFailed` - 库加载失败
	pub fn new(path: PathBuf) -> Result<Self, Error> {
		let name = path
			.file_name()
			.ok_or_else(|| Error::InvalidPath(path.clone()))?
			.to_string_lossy()
			.to_string();

		let library = unsafe {
			Library::new(&path)
				.map_err(|source| Error::LoadFailed { path: path.clone(), source })?
		};

		Ok(Self { name, path, library: Arc::new(library) })
	}

	/// 获取库名称
	pub fn name(&self) -> &str {
		&self.name
	}

	/// 获取库路径
	pub fn path(&self) -> &Path {
		&self.path
	}

	/// 获取库句柄的引用
	pub fn library(&self) -> &Arc<Library> {
		&self.library
	}
}

/// 全局动态库存储
static LIBRARY_STORE: LazyLock<Mutex<HashMap<String, Arc<LibraryInfo>>>> =
	LazyLock::new(|| Mutex::new(HashMap::new()));

/// 动态库注册表
///
/// 提供全局的动态库管理功能。
///
/// # 特性
///
/// - 线程安全的全局注册表
/// - 支持加载、获取、卸载和重载动态库
/// - 自动管理库的生命周期
/// - 防止重复加载
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_library::LibraryRegistry;
/// use std::path::Path;
///
/// // 加载库
/// LibraryRegistry::load(Path::new("plugin.dll"))?;
///
/// // 获取库
/// if let Some(lib) = LibraryRegistry::get("plugin.dll") {
///     println!("库已加载: {}", lib.name());
/// }
///
/// // 检查库是否存在
/// if LibraryRegistry::contains("plugin.dll") {
///     println!("库存在");
/// }
///
/// // 列出所有库
/// let all = LibraryRegistry::list();
/// for name in all {
///     println!("已加载的库: {}", name);
/// }
///
/// // 卸载库
/// LibraryRegistry::unload("plugin.dll")?;
/// ```
pub struct LibraryRegistry;

impl LibraryRegistry {
	/// 加载动态库
	///
	/// 从指定路径加载动态库并添加到注册表中。如果库已存在，则返回错误。
	///
	/// # 参数
	///
	/// - `path` - 库文件路径
	///
	/// # 返回
	///
	/// - `Ok(())` - 成功加载
	/// - `Err(Error)` - 加载失败
	///
	/// # 错误
	///
	/// - `Error::Exists` - 库已存在
	/// - `Error::InvalidPath` - 路径无效
	/// - `Error::LoadFailed` - 库加载失败
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_library::LibraryRegistry;
	/// use std::path::Path;
	///
	/// LibraryRegistry::load(Path::new("plugin.dll"))?;
	/// ```
	pub fn load(path: &Path) -> Result<(), Error> {
		let info = LibraryInfo::new(path.to_path_buf())?;
		let name = info.name.clone();

		let mut store = LIBRARY_STORE.lock().unwrap();
		if store.contains_key(&name) {
			return Err(Error::Exists(name));
		}

		store.insert(name, Arc::new(info));
		Ok(())
	}

	/// 从字符串路径加载动态库
	///
	/// 便捷方法，接受字符串路径参数。
	///
	/// # 参数
	///
	/// - `path` - 库文件路径字符串
	///
	/// # 返回
	///
	/// - `Ok(())` - 成功加载
	/// - `Err(Error)` - 加载失败
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_library::LibraryRegistry;
	///
	/// LibraryRegistry::load_from_path("plugin.dll")?;
	/// ```
	pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<(), Error> {
		Self::load(path.as_ref())
	}

	/// 强制加载动态库
	///
	/// 如果库已存在，则先卸载再重新加载。
	///
	/// # 参数
	///
	/// - `path` - 库文件路径
	///
	/// # 返回
	///
	/// - `Ok(())` - 成功加载
	/// - `Err(Error)` - 加载失败
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_library::LibraryRegistry;
	/// use std::path::Path;
	///
	/// LibraryRegistry::load_or_replace(Path::new("plugin.dll"))?;
	/// ```
	pub fn load_or_replace(path: &Path) -> Result<(), Error> {
		let info = LibraryInfo::new(path.to_path_buf())?;
		let name = info.name.clone();

		let mut store = LIBRARY_STORE.lock().unwrap();
		store.insert(name, Arc::new(info));
		Ok(())
	}

	/// 获取动态库信息
	///
	/// 根据库名称获取库信息。
	///
	/// # 参数
	///
	/// - `name` - 库名称
	///
	/// # 返回
	///
	/// - `Some(Arc<LibraryInfo>)` - 找到库
	/// - `None` - 未找到库
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_library::LibraryRegistry;
	///
	/// if let Some(lib) = LibraryRegistry::get("plugin.dll") {
	///     println!("库路径: {:?}", lib.path());
	/// }
	/// ```
	pub fn get(name: &str) -> Option<Arc<LibraryInfo>> {
		let store = LIBRARY_STORE.lock().unwrap();
		store.get(name).cloned()
	}

	/// 检查库是否存在
	///
	/// # 参数
	///
	/// - `name` - 库名称
	///
	/// # 返回
	///
	/// - `true` - 库存在
	/// - `false` - 库不存在
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_library::LibraryRegistry;
	///
	/// if LibraryRegistry::contains("plugin.dll") {
	///     println!("库已加载");
	/// }
	/// ```
	pub fn contains(name: &str) -> bool {
		let store = LIBRARY_STORE.lock().unwrap();
		store.contains_key(name)
	}

	/// 列出所有已加载的库名称
	///
	/// # 返回
	///
	/// 所有已加载库的名称列表
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_library::LibraryRegistry;
	///
	/// let libraries = LibraryRegistry::list();
	/// for name in libraries {
	///     println!("已加载: {}", name);
	/// }
	/// ```
	pub fn list() -> Vec<String> {
		let store = LIBRARY_STORE.lock().unwrap();
		store.keys().cloned().collect()
	}

	/// 获取已加载库的数量
	///
	/// # 返回
	///
	/// 已加载库的数量
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_library::LibraryRegistry;
	///
	/// let count = LibraryRegistry::count();
	/// println!("已加载 {} 个库", count);
	/// ```
	pub fn count() -> usize {
		let store = LIBRARY_STORE.lock().unwrap();
		store.len()
	}

	/// 卸载动态库
	///
	/// 从注册表中移除指定的库。如果库正在被其他地方引用，则返回错误。
	///
	/// # 参数
	///
	/// - `name` - 库名称
	///
	/// # 返回
	///
	/// - `Ok(PathBuf)` - 成功卸载，返回库的路径
	/// - `Err(Error)` - 卸载失败
	///
	/// # 错误
	///
	/// - `Error::NotFound` - 库不存在
	/// - `Error::InUse` - 库正在使用中
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_library::LibraryRegistry;
	///
	/// let path = LibraryRegistry::unload("plugin.dll")?;
	/// println!("已卸载库: {:?}", path);
	/// ```
	pub fn unload(name: &str) -> Result<PathBuf, Error> {
		let mut store = LIBRARY_STORE.lock().unwrap();
		let lib_info = store.remove(name).ok_or_else(|| Error::NotFound(name.to_string()))?;

		// 检查是否还有其他引用
		if Arc::strong_count(&lib_info) > 1 {
			// 放回去
			store.insert(name.to_string(), lib_info);
			return Err(Error::InUse(name.to_string()));
		}

		Ok(lib_info.path.clone())
	}

	/// 强制卸载动态库
	///
	/// 即使库正在使用中也会卸载。使用时需要特别小心。
	///
	/// # 参数
	///
	/// - `name` - 库名称
	///
	/// # 返回
	///
	/// - `Ok(PathBuf)` - 成功卸载，返回库的路径
	/// - `Err(Error)` - 卸载失败
	///
	/// # 警告
	///
	/// 强制卸载可能导致悬空指针，仅在确定安全的情况下使用。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_library::LibraryRegistry;
	///
	/// let path = LibraryRegistry::unload_force("plugin.dll")?;
	/// ```
	pub fn unload_force(name: &str) -> Result<PathBuf, Error> {
		let mut store = LIBRARY_STORE.lock().unwrap();
		let lib_info = store.remove(name).ok_or_else(|| Error::NotFound(name.to_string()))?;
		Ok(lib_info.path.clone())
	}

	/// 重新加载动态库
	///
	/// 先卸载库，然后重新加载。用于热重载场景。
	///
	/// # 参数
	///
	/// - `name` - 库名称
	///
	/// # 返回
	///
	/// - `Ok(())` - 成功重载
	/// - `Err(Error)` - 重载失败
	///
	/// # 错误
	///
	/// - `Error::NotFound` - 库不存在
	/// - `Error::InUse` - 库正在使用中
	/// - `Error::LoadFailed` - 重新加载失败
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_library::LibraryRegistry;
	///
	/// LibraryRegistry::reload("plugin.dll")?;
	/// ```
	pub fn reload(name: &str) -> Result<(), Error> {
		let path = Self::unload(name)?;
		Self::load(&path)
	}

	/// 清空所有库
	///
	/// 卸载所有已加载的库。如果有库正在使用中，则返回错误。
	///
	/// # 返回
	///
	/// - `Ok(())` - 成功清空
	/// - `Err(Error)` - 清空失败
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_library::LibraryRegistry;
	///
	/// LibraryRegistry::clear()?;
	/// ```
	pub fn clear() -> Result<(), Error> {
		let names: Vec<String> = {
			let store = LIBRARY_STORE.lock().unwrap();
			store.keys().cloned().collect()
		};

		for name in names {
			Self::unload(&name)?;
		}

		Ok(())
	}

	/// 强制清空所有库
	///
	/// 即使库正在使用中也会卸载所有库。
	///
	/// # 警告
	///
	/// 强制清空可能导致悬空指针，仅在确定安全的情况下使用。
	pub fn clear_force() {
		let mut store = LIBRARY_STORE.lock().unwrap();
		store.clear();
	}
}

mod store;

use crate::Loader;
use crate::types::LoaderId;
use puniyu_error::registry::Error;
use std::sync::{Arc, LazyLock};
use store::LoaderStore;

static STORE: LazyLock<LoaderStore> = LazyLock::new(LoaderStore::new);

/// 加载器注册表
///
/// 提供全局的加载器注册和管理功能。
///
/// # 特性
///
/// - 线程安全的全局注册表
/// - 支持通过索引或名称查找加载器
/// - 自动分配唯一索引
/// - 防止重复注册
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_loader::{Loader, LoaderRegistry};
/// use std::sync::Arc;
///
/// // 注册加载器
/// let loader = Arc::new(MyLoader);
/// let index = LoaderRegistry::register(loader)?;
///
/// // 通过索引获取
/// let loaders = LoaderRegistry::get(index);
///
/// // 通过名称获取
/// let loaders = LoaderRegistry::get("my_loader");
///
/// // 获取所有加载器
/// let all = LoaderRegistry::all();
///
/// // 注销加载器
/// LoaderRegistry::unregister(index)?;
/// ```
pub struct LoaderRegistry;

impl<'l> LoaderRegistry {
	/// 注册加载器
	///
	/// 将加载器添加到全局注册表中，返回分配的唯一索引。
	///
	/// # 参数
	///
	/// - `loader` - 要注册的加载器实例
	///
	/// # 返回
	///
	/// - `Ok(u64)` - 成功注册，返回分配的索引
	/// - `Err(Error)` - 注册失败（如加载器已存在）
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let loader = Arc::new(MyLoader);
	/// let index = LoaderRegistry::register(loader)?;
	/// ```
	pub fn register(loader: Arc<dyn Loader>) -> Result<u64, Error> {
		STORE.insert(loader)
	}

	/// 注销加载器
	///
	/// 从注册表中移除指定的加载器。
	///
	/// # 参数
	///
	/// - `loader` - 加载器标识符（索引或名称）
	///
	/// # 返回
	///
	/// - `Ok(())` - 成功注销
	/// - `Err(Error)` - 注销失败（如加载器不存在）
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// // 通过索引注销
	/// LoaderRegistry::unregister(42u64)?;
	///
	/// // 通过名称注销
	/// LoaderRegistry::unregister("my_loader")?;
	/// ```
	pub fn unregister<L>(loader: L) -> Result<(), Error>
	where
		L: Into<LoaderId<'l>>,
	{
		let loader = loader.into();
		match loader {
			LoaderId::Index(index) => Self::unregister_with_index(index),
			LoaderId::Name(name) => Self::unregister_with_loader_name(name.as_ref()),
		}
	}

	/// 通过索引注销加载器
	///
	/// # 参数
	///
	/// - `index` - 加载器的索引
	///
	/// # 返回
	///
	/// - `Ok(())` - 成功注销
	/// - `Err(Error)` - 加载器不存在
	pub fn unregister_with_index(index: u64) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if map.remove(&index).is_some() {
			Ok(())
		} else {
			Err(Error::NotFound("Loader".to_string()))
		}
	}

	/// 通过名称注销加载器
	///
	/// 移除所有匹配指定名称的加载器。
	///
	/// # 参数
	///
	/// - `name` - 加载器的名称
	///
	/// # 返回
	///
	/// - `Ok(())` - 成功注销
	pub fn unregister_with_loader_name(name: &str) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		map.retain(|_, h| h.name() != name);
		Ok(())
	}

	/// 获取加载器
	///
	/// 根据标识符获取加载器实例。
	///
	/// # 参数
	///
	/// - `loader` - 加载器标识符（索引或名称）
	///
	/// # 返回
	///
	/// 匹配的加载器列表（索引查找返回 0 或 1 个，名称查找可能返回多个）
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// // 通过索引获取
	/// let loaders = LoaderRegistry::get(42u64);
	///
	/// // 通过名称获取
	/// let loaders = LoaderRegistry::get("my_loader");
	/// ```
	pub fn get<L>(loader: L) -> Vec<Arc<dyn Loader>>
	where
		L: Into<LoaderId<'l>>,
	{
		let loader = loader.into();
		match loader {
			LoaderId::Index(index) => Self::get_with_index(index).into_iter().collect(),
			LoaderId::Name(name) => Self::get_with_loader_name(name.as_ref()),
		}
	}

	/// 通过索引获取加载器
	///
	/// # 参数
	///
	/// - `index` - 加载器的索引
	///
	/// # 返回
	///
	/// - `Some(Arc<dyn Loader>)` - 找到加载器
	/// - `None` - 未找到加载器
	pub fn get_with_index(index: u64) -> Option<Arc<dyn Loader>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	/// 通过名称获取加载器
	///
	/// 返回所有匹配指定名称的加载器。
	///
	/// # 参数
	///
	/// - `name` - 加载器的名称
	///
	/// # 返回
	///
	/// 匹配的加载器列表
	pub fn get_with_loader_name(name: &str) -> Vec<Arc<dyn Loader>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|h| h.name() == name).cloned().collect()
	}

	/// 获取所有加载器
	///
	/// 返回注册表中的所有加载器实例。
	///
	/// # 返回
	///
	/// 所有已注册的加载器列表
	pub fn all() -> Vec<Arc<dyn Loader>> {
		STORE.all()
	}
}

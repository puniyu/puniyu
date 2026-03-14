mod store;

use crate::Adapter;
use crate::types::AdapterId;
use puniyu_error::registry::Error;
use std::sync::{Arc, LazyLock};
use store::AdapterStore;

static STORE: LazyLock<AdapterStore> = LazyLock::new(AdapterStore::new);

/// 适配器注册表
///
/// 提供适配器的注册、卸载和查询功能。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_adapter::registry::AdapterRegistry;
///
/// // 注册适配器
/// let index = AdapterRegistry::register(adapter)?;
///
/// // 查询适配器
/// let adapters = AdapterRegistry::all();
///
/// // 卸载适配器
/// AdapterRegistry::unregister(index)?;
/// ```
pub struct AdapterRegistry;

impl<'a> AdapterRegistry {
	/// 注册一个适配器
	///
	/// # 参数
	///
	/// - `adapter` - 适配器信息
	///
	/// # 返回值
	///
	/// 返回适配器的索引 ID
	///
	/// # 错误
	///
	/// 如果适配器已存在，返回错误
	pub fn register(adapter: Arc<dyn Adapter>) -> Result<u64, Error> {
		STORE.insert(adapter)
	}

	/// 卸载一个适配器
	///
	/// # 参数
	///
	/// - `adapter` - 适配器 ID（索引或名称）
	///
	/// # 错误
	///
	/// 如果适配器不存在，返回错误
	pub fn unregister<A>(adapter: A) -> Result<(), Error>
	where
		A: Into<AdapterId<'a>>,
	{
		let adapter = adapter.into();
		match adapter {
			AdapterId::Index(index) => Self::unregister_with_index(index),
			AdapterId::Name(name) => Self::unregister_with_adapter_name(name),
		}
	}

	/// 通过索引卸载适配器
	pub fn unregister_with_index(index: u64) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if map.get(&index).is_none() {
			return Err(Error::NotFound("Adapter".to_string()));
		}
		map.remove(&index);
		Ok(())
	}

	/// 通过名称卸载适配器
	pub fn unregister_with_adapter_name(name: &str) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if !map.values().any(|v| v.name() == name) {
			return Err(Error::NotFound("Adapter".to_string()));
		}
		map.retain(|_, v| v.name() != name);
		Ok(())
	}

	/// 查询适配器
	///
	/// # 参数
	///
	/// - `adapter` - 适配器 ID（索引或名称）
	///
	/// # 返回值
	///
	/// 返回匹配的适配器信息列表
	pub fn get<A>(adapter: A) -> Vec<Arc<dyn Adapter>>
	where
		A: Into<AdapterId<'a>>,
	{
		let adapter = adapter.into();
		match adapter {
			AdapterId::Index(index) => Self::get_with_index(index).into_iter().collect(),
			AdapterId::Name(name) => Self::get_with_adapter_name(name),
		}
	}

	/// 通过索引查询适配器
	pub fn get_with_index(index: u64) -> Option<Arc<dyn Adapter>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	/// 通过名称查询适配器
	pub fn get_with_adapter_name(name: &str) -> Vec<Arc<dyn Adapter>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.name() == name).cloned().collect()
	}

	/// 获取所有适配器
	pub fn all() -> Vec<Arc<dyn Adapter>> {
		STORE.all()
	}
}

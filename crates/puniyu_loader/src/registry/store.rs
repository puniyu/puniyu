use crate::Loader;
use puniyu_error::registry::Error;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

/// 全局加载器索引计数器
static LOADER_INDEX: AtomicU64 = AtomicU64::new(0);

/// 加载器存储
///
/// 内部存储结构，用于管理加载器实例的映射关系。
#[derive(Clone, Default)]
pub(crate) struct LoaderStore(Arc<RwLock<HashMap<u64, Arc<dyn Loader>>>>);

impl LoaderStore {
	/// 创建新的加载器存储
	pub fn new() -> Self {
		Self::default()
	}

	/// 插入加载器
	///
	/// 将加载器添加到存储中，并分配唯一索引。
	///
	/// # 参数
	///
	/// - `loader` - 要插入的加载器实例
	///
	/// # 返回
	///
	/// - `Ok(u64)` - 成功插入，返回分配的索引
	/// - `Err(Error)` - 插入失败（加载器已存在）
	pub fn insert(&self, loader: Arc<dyn Loader>) -> Result<u64, Error> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v == &loader) {
			return Err(Error::Exists("Loader".to_string()));
		}
		let index = LOADER_INDEX.fetch_add(1, Ordering::Relaxed);
		map.insert(index, loader);
		Ok(index)
	}

	/// 获取所有加载器
	///
	/// # 返回
	///
	/// 所有已存储的加载器列表
	pub fn all(&self) -> Vec<Arc<dyn Loader>> {
		let map = &self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	/// 获取原始存储的引用
	///
	/// 用于需要直接访问底层 HashMap 的场景。
	pub(crate) fn raw(&self) -> Arc<RwLock<HashMap<u64, Arc<dyn Loader>>>> {
		self.0.clone()
	}
}

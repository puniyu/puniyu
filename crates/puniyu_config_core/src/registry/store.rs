use crate::Config;
use crate::types::ConfigInfo;
use puniyu_error::registry::Error;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

/// 全局配置 ID 计数器
static CONFIG_ID: AtomicU64 = AtomicU64::new(0);

/// 配置存储
///
/// 内部使用的配置存储结构，提供线程安全的配置管理。
#[derive(Default)]
pub(crate) struct ConfigStore(Arc<RwLock<HashMap<u64, ConfigInfo>>>);

impl ConfigStore {
	/// 创建新的配置存储
	pub fn new() -> Self {
		Self::default()
	}

	/// 插入配置
	///
	/// 将配置添加到存储中，如果配置已存在则返回错误。
	///
	/// # 参数
	///
	/// - `config`: 要插入的配置，可以是任何实现了 `Into<ConfigInfo>` 的类型
	///
	/// # 返回值
	///
	/// 成功时返回配置的唯一索引 ID，失败时返回错误
	pub fn insert<C>(&self, config: C) -> Result<u64, Error>
	where
		C: Into<ConfigInfo>,
	{
		let config = config.into();
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v == &config) {
			return Err(Error::Exists("Config".to_string()));
		}
		let index = CONFIG_ID.fetch_add(1, Ordering::Relaxed);
		map.insert(index, config).ok_or(Error::Exists("Config".to_string()))?;
		Ok(index)
	}

	/// 获取所有配置
	///
	/// # 返回值
	///
	/// 返回所有已注册配置的列表
	pub fn all(&self) -> Vec<ConfigInfo> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	/// 获取原始存储的引用
	///
	/// 用于内部访问底层的 HashMap。
	pub(crate) fn raw(&self) -> Arc<RwLock<HashMap<u64, ConfigInfo>>> {
		self.0.clone()
	}
}


use crate::Bot;
use puniyu_error::registry::Error;
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};

static BOT_INDEX: AtomicU64 = AtomicU64::new(0);

/// 机器人存储
///
/// 内部使用的机器人存储结构，提供线程安全的机器人实例管理。
#[derive(Clone, Default)]
pub(crate) struct BotStore(Arc<RwLock<HashMap<u64, Arc<dyn Bot>>>>);

impl BotStore {
	/// 创建新的机器人存储
	pub fn new() -> Self {
		Self::default()
	}

	/// 插入机器人实例
	///
	/// # 参数
	///
	/// - `bot` - 要插入的机器人实例
	///
	/// # 返回值
	///
	/// 返回分配的索引号，如果机器人已存在则返回错误
	pub fn insert(&self, bot: Arc<dyn Bot>) -> Result<u64, Error> {
		let index = BOT_INDEX.fetch_add(1, Ordering::SeqCst);
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v == &bot) {
			return Err(Error::Exists("Bot".to_string()));
		}
		map.insert(index, bot);
		Ok(index)
	}

	/// 获取原始存储的引用
	///
	/// 用于直接访问底层的 HashMap
	pub(crate) fn raw(&self) -> Arc<RwLock<HashMap<u64, Arc<dyn Bot>>>> {
		self.0.clone()
	}

	/// 获取所有机器人实例
	///
	/// # 返回值
	///
	/// 返回所有机器人实例的列表
	pub fn all(&self) -> Vec<Arc<dyn Bot>> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}
}

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Registry<V> {
	inner: Arc<RwLock<BTreeMap<u64, V>>>,
}

impl<V> Default for Registry<V> {
	fn default() -> Self {
		Self { inner: Arc::new(RwLock::new(BTreeMap::new())) }
	}
}

impl<V> Registry<V> {
	/// 创建空注册表。
	pub fn new() -> Self {
		Self::default()
	}

	/// 插入一个值，返回分配的 ID
	pub fn insert(&self, value: V) -> u64 {
		let mut map = self.inner.write().expect("poisoned lock");
		let id = map.last_key_value().map(|(k, _)| k.wrapping_add(1)).unwrap_or(0);
		map.insert(id, value);
		id
	}

	/// 按 ID 获取值的克隆。
	pub fn get(&self, id: u64) -> Option<V>
	where
		V: Clone,
	{
		self.inner.read().expect("poisoned lock").get(&id).cloned()
	}

	/// 按 ID 移除并返回值。
	pub fn remove(&self, id: u64) -> Option<V> {
		self.inner.write().expect("poisoned lock").remove(&id)
	}

	/// 是否包含某个 ID
	pub fn contains_key(&self, id: u64) -> bool {
		self.inner.read().expect("poisoned lock").contains_key(&id)
	}

	/// 返回条目数量。
	pub fn len(&self) -> usize {
		self.inner.read().expect("poisoned lock").len()
	}

	/// 是否为空。
	pub fn is_empty(&self) -> bool {
		self.inner.read().expect("poisoned lock").is_empty()
	}

	/// 清空所有条目。
	pub fn clear(&self) {
		self.inner.write().expect("poisoned lock").clear();
	}

	/// 获取所有值的克隆。
	pub fn values(&self) -> Vec<V>
	where
		V: Clone,
	{
		self.inner.read().expect("poisoned lock").values().cloned().collect()
	}

	/// 获取所有 ID。
	pub fn keys(&self) -> Vec<u64> {
		self.inner.read().expect("poisoned lock").keys().copied().collect()
	}

	/// 迭代：对每个 (id, value) 执行闭包。
	pub fn iter<F>(&self, mut f: F)
	where
		F: FnMut(u64, &V),
	{
		let map = self.inner.read().expect("poisoned lock");
		for (id, value) in map.iter() {
			f(*id, value);
		}
	}

	/// 就地修改指定 ID 的值。闭包接收 `&mut V`。
	pub fn get_mut<F>(&self, id: u64, f: F) -> bool
	where
		F: FnOnce(&mut V),
	{
		let mut map = self.inner.write().expect("poisoned lock");
		if let Some(entry) = map.get_mut(&id) {
			f(entry);
			true
		} else {
			false
		}
	}

	/// 如果 ID 不存在则插入，存在则更新。返回 ID。
	pub fn upsert(&self, id: u64, value: V) -> u64 {
		let mut map = self.inner.write().expect("poisoned lock");
		map.insert(id, value);
		id
	}

	/// 就地迭代：对每个 (id, value) 执行可变闭包。
	pub fn iter_mut<F>(&self, mut f: F)
	where
		F: FnMut(u64, &mut V),
	{
		let mut map = self.inner.write().expect("poisoned lock");
		for (id, value) in map.iter_mut() {
			f(*id, value);
		}
	}

	/// 保留满足谓词的条目，移除其余
	pub fn retain<F>(&self, mut f: F)
	where
		F: FnMut(u64, &V) -> bool,
	{
		self.inner.write().expect("poisoned lock").retain(|id, value| f(*id, value));
	}
}

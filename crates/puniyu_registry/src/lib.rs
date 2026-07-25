use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

pub use scc::hash_map::Entry;
use scc::HashMap;


pub struct Registry<V> {
    map: HashMap<u64, V>,
    next_id: AtomicU64,
}

impl<V> Default for Registry<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> Registry<V> {
    #[inline]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            next_id: AtomicU64::new(0),
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
            next_id: AtomicU64::new(0),
        }
    }
}

impl<V> Registry<V> {
    /// 插入键值对，返回被替换的旧值
    #[inline]
    pub fn insert(&self, id: u64, value: V) -> Option<V> {
        match self.map.entry_sync(id) {
            Entry::Occupied(mut entry) => Some(entry.insert(value)),
            Entry::Vacant(entry) => {
                entry.insert_entry(value);
                None
            }
        }
    }

    /// 获取指定键的 Entry，用于就地操作
    #[inline]
    pub fn entry(&self, key: u64) -> Entry<'_, u64, V> {
        self.map.entry_sync(key)
    }
}

impl<V: Clone> Registry<V> {
    /// 按键获取值的克隆
    #[inline]
    pub fn get(&self, id: u64) -> Option<V> {
        self.map.read_sync(&id, |_, v| v.clone())
    }

    /// 按键获取键值对的克隆
    #[inline]
    pub fn get_key_value(&self, id: u64) -> Option<(u64, V)> {
        self.map.read_sync(&id, |&k, v| (k, v.clone()))
    }
}

impl<V> Registry<V> {
    /// 按键获取值的可变引用，通过回调修改
    #[inline]
    pub fn get_mut<R, F>(&self, id: u64, f: F) -> Option<R>
    where
        F: FnOnce(&mut V) -> R,
    {
        self.map.update_sync(&id, |_, v| f(v))
    }

    /// 是否包含指定键
    #[inline]
    pub fn contains_key(&self, id: u64) -> bool {
        self.map.contains_sync(&id)
    }
}

impl<V> Registry<V> {
    /// 移除指定键的条目，返回值
    #[inline]
    pub fn remove(&self, id: u64) -> Option<V> {
        self.map.remove_sync(&id).map(|(_, v)| v)
    }

    /// 移除指定键的条目，返回键值对
    #[inline]
    pub fn remove_entry(&self, id: u64) -> Option<(u64, V)> {
        self.map.remove_sync(&id)
    }

    /// 保留满足谓词的条目，移除其余
    #[inline]
    pub fn retain<F>(&self, mut f: F)
    where
        F: FnMut(u64, &V) -> bool,
    {
        self.map.retain_sync(|&id, value| f(id, value));
    }

    /// 清空所有条目
    #[inline]
    pub fn clear(&self) {
        self.map.clear_sync();
    }
}

impl<V> Registry<V> {
    /// 返回条目数量
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// 是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<V> Registry<V> {
    /// 对每个键值对执行闭包
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(u64, &V),
    {
        self.map.iter_sync(|&id, value| {
            f(id, value);
            true
        });
    }

    /// 返回所有键的快照
    pub fn keys(&self) -> Vec<u64> {
        let mut result = Vec::new();
        self.map.iter_sync(|&k, _| { result.push(k); true });
        result
    }

    /// 返回所有值的克隆快照
    pub fn values(&self) -> Vec<V>
    where
        V: Clone,
    {
        let mut result = Vec::new();
        self.map.iter_sync(|_, v| { result.push(v.clone()); true });
        result
    }

    /// 返回所有键值对的克隆快照
    pub fn iter(&self) -> Vec<(u64, V)>
    where
        V: Clone,
    {
        let mut result = Vec::new();
        self.map.iter_sync(|&k, v| { result.push((k, v.clone())); true });
        result
    }
}

impl<V: Clone> Clone for Registry<V> {
    fn clone(&self) -> Self {
        let reg = Self::new();
        self.map.iter_sync(|&k, v| {
            reg.map.insert_sync(k, v.clone()).ok();
            true
        });
        let next = self.next_id.load(Ordering::Relaxed);
        reg.next_id.store(next, Ordering::Relaxed);
        reg
    }
}

impl<V: fmt::Debug + Clone> fmt::Debug for Registry<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("Registry");
        let mut entries = Vec::new();
        self.map.iter_sync(|&k, v| {
            entries.push((k, v.clone()));
            true
        });
        debug.field("len", &entries.len());
        debug.field("entries", &entries);
        debug.finish()
    }
}

impl<V: PartialEq> PartialEq for Registry<V> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.map.iter_sync(|&k, v| {
            other.map.read_sync(&k, |_, ov| v == ov).unwrap_or(false)
        })
    }
}

impl<V: Eq> Eq for Registry<V> {}

impl<V> FromIterator<(u64, V)> for Registry<V> {
    fn from_iter<I: IntoIterator<Item = (u64, V)>>(iter: I) -> Self {
        let reg = Self::new();
        for (k, v) in iter {
            reg.map.insert_sync(k, v).ok();
            let _ = reg.next_id.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                if k >= current {
                    Some(k + 1)
                } else {
                    None
                }
            });
        }
        reg
    }
}

impl<V> Extend<(u64, V)> for Registry<V> {
    fn extend<I: IntoIterator<Item = (u64, V)>>(&mut self, iter: I) {
        for (k, v) in iter {
            self.map.insert_sync(k, v).ok();
            let _ = self.next_id.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                if k >= current {
                    Some(k + 1)
                } else {
                    None
                }
            });
        }
    }
}

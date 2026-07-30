use std::sync::Arc;

use puniyu_bot::Bot;
use puniyu_registry::Registry;

#[derive(Default)]
pub struct BotContext {
    inner: Registry<Arc<dyn Bot>>
}

impl BotContext {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// 查询指定 ID 的 Bot
    pub fn get(&self, id: &str) -> Option<Arc<dyn Bot>> {
        let mut result = None;
        self.inner.for_each(|_, b| {
            if b.self_id() == id {
                result = Some(b.clone());
            }
        });
        result
    }

    /// 获取所有 Bot
    pub fn values(&self) -> Vec<Arc<dyn Bot>> {
        self.inner.values()
    }

    /// 注册 Bot
    pub fn insert<B: Bot + 'static>(&self, bot: B) {
        self.inner.insert(Arc::new(bot));
    }

    /// 注销 Bot
    pub fn remove(&self, id: &str) -> Option<Arc<dyn Bot>> {
        let mut key = None;
        self.inner.for_each(|k, b| {
            if b.self_id() == id {
                key = Some(k);
            }
        });
        key.and_then(|k| self.inner.remove(k))
    }
}
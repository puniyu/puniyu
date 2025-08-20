use emitter_rs::EventEmitter as Emitter;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub struct EventEmitter {
    emitter: Arc<Mutex<Emitter>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            emitter: Arc::new(Mutex::new(Emitter::new())),
        }
    }

    /// 触发事件
    pub fn emit<T>(&self, event: &str, data: T)
    where
        T: Serialize,
    {
        if let Ok(mut emitter) = self.emitter.lock() {
            emitter.emit(event, data);
        }
    }

    /// 注册事件监听
    pub fn on<F, T>(&self, event: &str, callback: F)
    where
        F: Fn(T) + 'static + Send + Sync,
        for<'de> T: Deserialize<'de>,
    {
        if let Ok(mut emitter) = self.emitter.lock() {
            emitter.on(event, callback);
        }
    }

    /// 注册一次事件监听
    pub fn once<F, T>(&self, event: &str, callback: F)
    where
        F: Fn(T) + 'static + Send + Sync,
        for<'de> T: Deserialize<'de>,
    {
        if let Ok(mut emitter) = self.emitter.lock() {
            emitter.once(event, callback);
        }
    }
}

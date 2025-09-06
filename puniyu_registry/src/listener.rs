use emitter_rs::EventEmitter as Emitter;
use serde::Deserialize;
use std::sync::{Arc, LazyLock, OnceLock, RwLock};

static EMITTER: LazyLock<Arc<RwLock<Emitter>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Emitter::new())));

/// 注册事件监听
pub fn on<F, T>(event: &str, callback: F)
where
    F: Fn(T) + 'static + Send + Sync,
    for<'de> T: Deserialize<'de>,
{
    if let Ok(mut emitter_guard) = EMITTER.write() {
        emitter_guard.on(event, callback);
    }
}

/// 触发事件
pub fn emit<T>(event: &str, data: T)
where
    T: serde::Serialize,
{
    if let Ok(mut emitter_guard) = EMITTER.write() {
        emitter_guard.emit(event, data);
    }
}

/// 注册一次事件监听
pub fn once<F, T>(event: &str, callback: F)
where
    F: Fn(T) + 'static + Send + Sync,
    for<'de> T: Deserialize<'de>,
{
    if let Ok(mut emitter_guard) = EMITTER.write() {
        emitter_guard.once(event, callback);
    }
}

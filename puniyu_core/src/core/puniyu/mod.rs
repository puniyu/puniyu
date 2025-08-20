use super::listeners::EventEmitter;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

static LISTENERS: LazyLock<EventEmitter> = LazyLock::new(EventEmitter::new);

pub struct Puniyu;
impl Puniyu {
    /// 触发事件
    ///
    /// # 示例
    ///
    /// ```
    /// use puniyu_core::puniyu;
    ///
    /// puniyu::emit("event", "data");
    /// ```
    pub fn emit<T>(event: &str, data: T)
    where
        T: Serialize,
    {
        LISTENERS.emit(event, data)
    }

    /// 监听事件
    ///
    /// # 示例
    ///
    /// ```
    /// use puniyu_core::puniyu;
    ///
    /// puniyu::on("event", |data: String| {
    ///     println!("{}", data);
    /// });
    /// ```
    ///
    pub fn on<F, T>(event: &str, callback: F)
    where
        for<'de> T: Deserialize<'de>,
        F: Fn(T) + 'static + Sync + Send,
    {
        LISTENERS.on(event, callback)
    }

    /// 监听一次事件
    ///
    /// # 示例
    ///
    /// ```
    /// use puniyu_core::puniyu;
    ///
    /// puniyu::once("event", |data: String| {
    ///     println!("{}", data);
    /// });
    /// ```
    pub fn once<F, T>(event: &str, callback: F)
    where
        for<'de> T: Deserialize<'de>,
        F: Fn(T) + 'static + Sync + Send,
    {
        LISTENERS.once(event, callback)
    }
}

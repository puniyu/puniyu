//! # puniyu_handler
//!
//! 处理器库，提供统一的 `Handler` 接口。
//!
//! ## 特性
//!
//! - `Handler` trait 定义事件处理模型
//! - 支持处理 `puniyu_event::Event`
//! - 支持前置、后置和短路的洋葱调用链
//! - 支持优先级排序

mod types;
#[doc(inline)]
pub use types::*;

use async_trait::async_trait;

/// 事件处理器接口。
#[async_trait]
pub trait Handler: Send + Sync {
    /// 获取处理器名称。
    fn name(&self) -> &str;

    /// 获取处理器优先级。数值越小越先执行。默认 500。
    fn priority(&self) -> u32 {
        500
    }

    /// 处理事件。
    ///
    /// 调用 [`HandlerContext::next`] 进入后续处理器；不调用则正常终止调用链。
    async fn handle(&self, ctx: HandlerContext<'_>);
}

impl PartialEq for dyn Handler {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for dyn Handler {}

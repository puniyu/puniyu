//! # puniyu_middleware
//!
//! 中间件库，提供统一的 `Middleware` 接口。
//!
//! ## 特性
//!
//! - `Middleware` trait 定义事件处理模型
//! - 支持处理 `puniyu_event::Event`
//! - 支持前置、后置和短路的洋葱调用链
//! - 支持优先级排序

mod types;
#[doc(inline)]
pub use types::*;

use async_trait::async_trait;

/// 中间件接口。
#[async_trait]
pub trait Middleware: Send + Sync {
    /// 获取中间件名称。
    fn name(&self) -> &str;

    /// 获取中间件优先级。数值越小越先执行。默认 500。
    fn priority(&self) -> u32 {
        500
    }

    /// 处理事件。
   ///
    /// 调用 [`MiddlewareContext::next`] 进入后续中间件；不调用则正常终止调用链。
    async fn handle(&self, ctx: MiddlewareContext<'_>);
}

impl PartialEq for dyn Middleware {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for dyn Middleware {}

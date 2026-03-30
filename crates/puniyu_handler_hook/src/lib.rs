//! # puniyu_handler_hook
//!
//! puniyu Hook 处理器，将事件分发到所有已注册的 Hook。
//!
//! ## 示例
//!
//! ```rust,ignore
//! use puniyu_handler::Handler;
//! use puniyu_handler_hook::Handler as HookHandler;
//!
//! let handler = HookHandler;
//! assert_eq!(handler.name(), "hook");
//! ```

mod handler;
#[doc(inline)]
pub use handler::HookHandler as Handler;

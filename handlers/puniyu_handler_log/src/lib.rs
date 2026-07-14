//! # puniyu_handler_log
//!
//! 提供基于洋葱调用链的事件日志处理器。
//!
//! ```rust,ignore
//! use puniyu_handler_log::LogHandler;
//!
//! let handler = LogHandler::new();
//! ```

mod message;

#[doc(inline)]
pub use message::LogHandler;

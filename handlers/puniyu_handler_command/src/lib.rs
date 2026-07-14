//! # puniyu_handler_command
//!
//! 统一完成命令激活、解析、权限、冷却与执行分发。
//!
//! ```rust,ignore
//! use puniyu_handler_command::CommandHandler;
//!
//! let handler = CommandHandler::new();
//! ```

mod cooldown;
mod executor;
mod handler;
mod invocation;
mod policy;

#[doc(inline)]
pub use handler::CommandHandler;

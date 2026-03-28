//! # puniyu_handler_command
//!
//! 统一的 puniyu 命令处理器，覆盖命令匹配、权限检查、冷却控制与执行分发场景。
//!
//! ## 特性
//!
//! - 提供 [`CommandHandler`]
//! - 结合 `puniyu_command_parser` 完成命令文本解析
//! - 支持权限检查、响应模式判断和主人命令校验
//! - 支持全局、Bot、好友、群组与群成员冷却
//!
//! ## 示例
//!
//! ```rust,ignore
//! use puniyu_handler::Handler;
//! use puniyu_handler_command::CommandHandler;
//!
//! let handler = CommandHandler::default();
//! assert_eq!(handler.name(), "command");
//! ```

mod config;
mod handler;
mod message;
mod tools;

pub use handler::CommandHandler;

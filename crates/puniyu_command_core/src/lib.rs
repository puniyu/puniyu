//! # puniyu_command_core
//!
//! 命令核心库，提供命令参数、权限和动作的类型定义。
//!
//! ## 概述
//!
//! `puniyu_command_core` 定义了命令系统的核心类型，包括：
//!
//! - 命令参数定义和值类型
//! - 命令权限控制
//! - 命令执行动作
//!
//! ## 主要类型
//!
//! - [`Arg`] - 命令参数定义
//! - [`ArgValue`] - 参数值
//! - [`ArgType`] - 参数类型（字符串、整数、浮点数、布尔值）
//! - [`ArgMode`] - 参数模式（位置参数、命名参数）
//! - [`Permission`] - 权限类型（所有人、仅主人）
//! - [`CommandAction`] - 命令执行动作（完成、继续）
//!
//! ## 快速开始
//!
//! ### 定义命令参数
//!
//! ```rust
//! use puniyu_command_core::{Arg, ArgType};
//!
//! // 字符串参数
//! let name_arg = Arg::string("name").required();
//!
//! // 整数参数
//! let count_arg = Arg::int("count").optional();
//!
//! // 命名参数
//! let flag_arg = Arg::bool("verbose").named();
//! ```
//!
//! ### 使用参数值
//!
//! ```rust
//! use puniyu_command_core::ArgValue;
//!
//! let value = ArgValue::from("hello");
//! if let Some(s) = value.as_str() {
//!     println!("字符串值: {}", s);
//! }
//! ```
//!
//! ### 权限控制
//!
//! ```rust
//! use puniyu_command_core::Permission;
//!
//! let permission = Permission::Master;
//! ```

mod types;
#[doc(inline)]
pub use types::*;

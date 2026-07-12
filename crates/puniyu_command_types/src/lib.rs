//! # puniyu_command_types
//!
//! 统一的 puniyu 命令类型库，覆盖命令参数、参数值、权限与执行动作场景。
//!
//! ## 特性
//!
//! - 提供 [`Arg`]、[`ArgValue`]、[`ElemType`]、[`ArgMode`]
//! - 提供 [`Permission`] 与 [`CommandAction`]
//! - 支持链式定义命令参数
//! - 支持 `serde` 序列化和 `strum` 字符串转换
//!
//! ## 示例
//!
//! ### 定义命令参数
//!
//! ```rust
//! use puniyu_command_types::{Arg, ArgMode, ElemType, ArgValue, CommandAction, Permission};
//!
//! let arg = Arg::string("name").required().description("用户名");
//! assert_eq!(arg.arg_type, ElemType::String);
//! assert_eq!(arg.mode, ArgMode::Positional);
//!
//! let value = ArgValue::from("Alice");
//! assert_eq!(value.as_str(), Some("Alice"));
//!
//! assert_eq!(Permission::default(), Permission::All);
//! assert_eq!(CommandAction::done().unwrap(), CommandAction::Done);
//! ```

mod action;
#[doc(inline)]
pub use action::*;
mod arg;
#[doc(inline)]
pub use arg::*;
mod permission;
#[doc(inline)]
pub use permission::Permission;

//! # puniyu_command
//!
//! 命令库，提供命令定义和注册管理。
//!
//! ## 概述
//!
//! `puniyu_command` 是 Puniyu 框架的命令系统，提供了命令的定义、注册和管理功能。
//! 该库基于 trait 的设计，允许开发者轻松创建自定义命令。
//!
//! ## 主要功能
//!
//! - 命令定义 - 通过实现 `Command` trait 定义命令
//! - 命令注册 - 使用 `CommandRegistry` 注册和管理命令
//! - 参数解析 - 支持位置参数和命名参数
//! - 权限控制 - 内置权限系统
//! - 优先级调度 - 支持命令优先级
//!
//! ## 快速开始
//!
//! ### 定义命令
//!
//! ```rust,ignore
//! use puniyu_command::{Command, CommandAction, Arg, Permission};
//! use puniyu_context::MessageContext;
//! use async_trait::async_trait;
//!
//! struct HelloCommand;
//!
//! #[async_trait]
//! impl Command for HelloCommand {
//!     fn name(&self) -> &'static str {
//!         "hello"
//!     }
//!
//!     fn description(&self) -> Option<&'static str> {
//!         Some("打招呼命令")
//!     }
//!
//!     fn args(&self) -> Vec<Arg<'static>> {
//!         vec![Arg::string("name").required()]
//!     }
//!
//!     async fn run(&self, ctx: &MessageContext) -> Result<CommandAction> {
//!         // 命令逻辑
//!         Ok(CommandAction::Done)
//!     }
//! }
//! ```
//!
//! ### 注册命令
//!
//! ```rust,ignore
//! use puniyu_command::CommandRegistry;
//!
//! // 注册命令
//! CommandRegistry::register(command_info)?;
//! ```

#[cfg(feature = "registry")]
mod registry;

use async_trait::async_trait;
#[cfg(feature = "registry")]
pub use registry::CommandRegistry;
mod types;
#[doc(inline)]
pub use types::*;

use puniyu_context::MessageContext;

/// 命令 Trait
///
/// 定义命令的基本行为和接口。所有命令都必须实现此 trait。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_command::{Command, CommandAction, Arg};
/// use puniyu_context::MessageContext;
/// use async_trait::async_trait;
///
/// struct MyCommand;
///
/// #[async_trait]
/// impl Command for MyCommand {
///     fn name(&self) -> &'static str {
///         "mycommand"
///     }
///
///     async fn run(&self, ctx: &MessageContext) -> Result<CommandAction> {
///         println!("执行命令");
///         Ok(CommandAction::Done)
///     }
/// }
/// ```
#[async_trait]
pub trait Command: Send + Sync + 'static {
	/// 命令名称
	///
	/// 返回命令的唯一标识符。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// fn name(&self) -> &'static str {
	///     "hello"
	/// }
	/// ```
	fn name(&self) -> &'static str;

	/// 命令描述
	///	TODO: 实现插件帮助
	/// 返回命令的描述信息，用于帮助文档。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// fn description(&self) -> Option<&'static str> {
	///     Some("打招呼命令")
	/// }
	/// ```
	fn description(&self) -> Option<&'static str> {
		None
	}

	/// 参数列表
	///
	/// 返回命令接受的参数定义。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// fn args(&self) -> Vec<Arg<'static>> {
	///     vec![
	///         Arg::string("name").required(),
	///         Arg::int("age").optional(),
	///     ]
	/// }
	/// ```
	fn args(&'_ self) -> Vec<Arg<'static>> {
		Vec::new()
	}

	/// 命令优先级
	///
	/// 返回命令的执行优先级，数值越小优先级越高。
	/// 默认值为 500。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// fn rank(&self) -> u32 {
	///     100  // 高优先级
	/// }
	/// ```
	fn rank(&self) -> u32 {
		500
	}

	/// 命令别名
	///
	/// 返回命令的别名列表。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// fn alias(&self) -> Vec<&'static str> {
	///     vec!["hi", "hey"]
	/// }
	/// ```
	fn alias(&self) -> Vec<&'static str> {
		Vec::new()
	}

	/// 权限等级
	///
	/// 返回执行此命令所需的权限。
	/// 默认为 `Permission::All`（所有人可执行）。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// fn permission(&self) -> Permission {
	///     Permission::Master  // 仅主人可执行
	/// }
	/// ```
	fn permission(&self) -> Permission {
		Permission::All
	}

	/// 执行命令
	///
	/// 命令的主要逻辑实现。
	///
	/// # 参数
	///
	/// - `ctx` - 消息上下文，包含消息信息和参数
	///
	/// # 返回值
	///
	/// 返回 `CommandAction`，指示是否继续传播命令
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// async fn run(&self, ctx: &MessageContext) -> Result<CommandAction> {
	///     // 获取参数
	///     let name = ctx.get_arg("name")?;
	///     
	///     // 执行逻辑
	///     println!("Hello, {}!", name);
	///     
	///     // 返回完成
	///     Ok(CommandAction::Done)
	/// }
	/// ```
	async fn run(&self, ctx: &MessageContext) -> puniyu_error::Result<CommandAction>;
}

impl PartialEq for dyn Command {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name()
			&& self.description() == other.description()
			&& self.args() == other.args()
			&& self.rank() == other.rank()
			&& self.alias() == other.alias()
			&& self.permission() == other.permission()
	}
}

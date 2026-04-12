//! # puniyu_command
//!
//! 统一的 puniyu 命令库，覆盖命令定义、元信息与注册表管理场景。
//!
//! ## 特性
//!
//! - 提供 [`Command`] trait 定义命令行为
//! - 提供 `CommandRegistry` 管理命令注册与查询
//! - 复用 `puniyu_command_types` 中的参数、权限和动作类型
//! - 支持命令别名、优先级和权限控制
//!
//! ## 示例
//!
//! ```rust,ignore
//! use async_trait::async_trait;
//! use puniyu_command::{Arg, Command, CommandAction, Permission};
//! use puniyu_context::MessageContext;
//!
//! struct HelloCommand;
//!
//! #[async_trait]
//! impl Command for HelloCommand {
//!     fn name(&self) -> &'static str {
//!         "hello"
//!     }
//!
//!     fn args(&self) -> Vec<Arg<'static>> {
//!         vec![Arg::string("name").required()]
//!     }
//!
//!     fn permission(&self) -> Permission {
//!         Permission::All
//!     }
//!
//!     async fn run(&self, _ctx: &MessageContext) -> puniyu_error::Result<CommandAction> {
//!         CommandAction::done()
//!     }
//! }
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

/// 判断当前权限是否满足目标权限。
#[macro_export]
macro_rules! has_permission {
	($current:expr, $required:expr $(,)?) => {{
		matches!(
			($current, $required),
			(_, $crate::Permission::All)
				| ($crate::Permission::Admin, $crate::Permission::Admin)
				| ($crate::Permission::Owner, $crate::Permission::Admin)
				| ($crate::Permission::Owner, $crate::Permission::Owner)
				| ($crate::Permission::Master, $crate::Permission::Admin)
				| ($crate::Permission::Master, $crate::Permission::Owner)
				| ($crate::Permission::Master, $crate::Permission::Master)
		)
	}};
}

/// 命令行为接口。
#[async_trait]
pub trait Command: Send + Sync + 'static {
	/// 返回命令名称。
	fn name(&self) -> &'static str;

	/// 返回命令描述。
	fn description(&self) -> Option<&'static str> {
		None
	}

	/// 返回命令参数列表。
	fn args(&'_ self) -> Vec<Arg<'static>> {
		Vec::new()
	}

	/// 返回命令优先级，默认值为 `500`。
	fn priority(&self) -> u32 {
		500
	}

	/// 返回命令别名列表。
	fn alias(&self) -> Vec<&'static str> {
		Vec::new()
	}

	/// 返回命令权限，默认为 [`Permission::All`]。
	fn permission(&self) -> Permission {
		Permission::All
	}

	/// 执行命令。
	async fn run(&self, ctx: &MessageContext) -> puniyu_error::Result<CommandAction>;
}

impl PartialEq for dyn Command {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name()
			&& self.description() == other.description()
			&& self.args() == other.args()
			&& self.priority() == other.priority()
			&& self.alias() == other.alias()
			&& self.permission() == other.permission()
	}
}

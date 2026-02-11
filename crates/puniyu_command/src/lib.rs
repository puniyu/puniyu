#[cfg(feature = "registry")]
mod registry;

use async_trait::async_trait;
#[cfg(feature = "registry")]
pub use registry::CommandRegistry;
mod types;
#[doc(inline)]
pub use types::*;

use puniyu_context::MessageContext;

#[async_trait]
pub trait Command: Send + Sync + 'static {
	/// 命令名称
	fn name(&self) -> &'static str;

	/// 描述
	fn description(&self) -> Option<&'static str> {
		None
	}

	/// 参数列表
	fn args(&'_ self) -> Vec<Arg<'static>> {
		Vec::new()
	}

	/// 优先级
	fn rank(&self) -> u32 {
		500
	}

	/// 命令别名
	fn alias(&self) -> Vec<&'static str> {
		Vec::new()
	}

	/// 权限等级
	fn permission(&self) -> Permission {
		Permission::All
	}

	/// 执行的函数
	async fn run(&self, ctx: &MessageContext) -> puniyu_error::Result<CommandAction>;
}

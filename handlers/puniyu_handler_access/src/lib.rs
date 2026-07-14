//! # puniyu_handler_access
//!
//! 提供统一的消息黑白名单访问控制。
//!
//! ```rust,ignore
//! use puniyu_handler_access::AccessHandler;
//!
//! let handler = AccessHandler::new();
//! ```

use async_trait::async_trait;
use puniyu_config::{ListConfig, app::AppConfig};
use puniyu_handler::{HandleContext, Handler};

/// 好友、群组与频道消息访问控制处理器。
#[derive(Debug, Default, Clone, Copy)]
pub struct AccessHandler;

impl AccessHandler {
	/// 创建访问控制处理器。
	pub const fn new() -> Self {
		Self
	}
}

#[async_trait]
impl Handler for AccessHandler {
	fn name(&self) -> &'static str {
		"access"
	}

	async fn handle(&self, mut ctx: HandleContext<'_, '_>) {
		let Some(message) = ctx.as_message() else {
			ctx.next().await;
			return;
		};

		let app = AppConfig::get();
		let allowed = if let Some(group) = message.as_group() {
			is_allowed(&app.group(), group.group_id())
		} else if let Some(group) = message.as_group_temp() {
			is_allowed(&app.group(), group.group_id())
		} else if let Some(guild) = message.as_guild() {
			is_allowed(&app.group(), guild.guild_id())
		} else {
			is_allowed(&app.friend(), message.user_id())
		};

		if allowed {
			ctx.next().await;
		}
	}
}

fn is_allowed(config: &ListConfig, id: &str) -> bool {
	let enabled = config.enable_list();
	if !enabled.is_empty() {
		return enabled.contains(&id);
	}

	!config.disable_list().contains(&id)
}

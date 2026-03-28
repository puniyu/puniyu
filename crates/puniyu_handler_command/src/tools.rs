use puniyu_config::{ReactiveMode, app_config};
use puniyu_context::MessageContext;
use puniyu_event::EventBase;

pub mod cooldown;

/// 检查是否通过黑白名单
/// - 白名单优先：如果白名单不为空，只允许白名单内的 ID
/// - 黑名单：如果在黑名单内，则拒绝
fn check_list(id: &str, enable_list: &[String], disable_list: &[String]) -> bool {
	if !enable_list.is_empty() {
		return enable_list.contains(&id.to_string());
	}
	!disable_list.contains(&id.to_string())
}

/// 检查消息权限
pub fn check_perm(event: &MessageContext) -> bool {
	let config = app_config();
	if let Some(event) = event.as_group() {
		let group_id = event.group_id();
		let config = config.group();
		check_list(group_id, config.enable_list(), config.disable_list())
	} else {
		let user_id = event.user_id();
		let config = config.friend();
		check_list(user_id, config.enable_list(), config.disable_list())
	}
}

pub fn check_mode(event: &MessageContext, mode: &ReactiveMode, has_alias: bool) -> bool {
	match mode {
		ReactiveMode::All => true,
		ReactiveMode::AtBot => event.mentions_bot(),
		ReactiveMode::Alias => has_alias,
		ReactiveMode::AtOrAlias => event.mentions_bot() || has_alias,
		ReactiveMode::Master => event.is_master(),
	}
}

use puniyu_core::command::Permission;
use puniyu_core::config::{ReactiveMode, app_config};
use puniyu_core::context::MessageContext;
use puniyu_core::event::EventBase;

pub mod cooldown;

/// 检查是否通过黑白名单
/// - 白名单优先：如果白名单不为空，只允许白名单内的 ID
/// - 黑名单：如果在黑名单内，则拒绝
fn check_list(id: &str, enable_list: &[&str], disable_list: &[&str]) -> bool {
	if !enable_list.is_empty() {
		return enable_list.contains(&id);
	}
	!disable_list.contains(&id)
}

/// 检查消息权限
pub fn check_perm(event: &MessageContext) -> bool {
	let config = app_config();
	if let Some(event) = event.as_group() {
		let group_id = event.group_id();
		let config = config.group();
		check_list(group_id, config.enable_list().as_slice(), config.disable_list().as_slice())
	} else {
		let user_id = event.user_id();
		let config = config.friend();
		check_list(user_id, config.enable_list().as_slice(), config.disable_list().as_slice())
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

pub fn get_permission(event: &MessageContext) -> Permission {
	if event.is_master() {
		Permission::Master
	} else if let Some(group) = event.as_group() {
		if group.is_owner() {
			Permission::Owner
		} else if group.is_admin() {
			Permission::Admin
		} else {
			Permission::All
		}
	} else if let Some(group_temp) = event.as_group_temp() {
		if group_temp.is_owner() {
			Permission::Owner
		} else if group_temp.is_admin() {
			Permission::Admin
		} else {
			Permission::All
		}
	} else if let Some(guild) = event.as_guild() {
		if guild.is_owner() {
			Permission::Owner
		} else if guild.is_admin() {
			Permission::Admin
		} else {
			Permission::All
		}
	} else {
		Permission::All
	}
}

macro_rules! has_permission {
	($perm:expr, $required:expr $(,)?) => {
		$perm.satisfies($required)
	};
}
pub(crate) use has_permission;
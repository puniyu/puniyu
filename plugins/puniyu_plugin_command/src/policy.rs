use puniyu_command::Permission;
use puniyu_event::message::MessageEvent;

pub(crate) fn permission(message: &MessageEvent) -> Permission {
	if let Some(group) = message.as_group() {
		if group.is_owner() {
			Permission::Admin
		} else if group.is_admin() {
			Permission::Trusted
		} else {
			Permission::Member
		}
	} else if let Some(group) = message.as_group_temp() {
		if group.is_owner() {
			Permission::Admin
		} else if group.is_admin() {
			Permission::Trusted
		} else {
			Permission::Member
		}
	} else if let Some(guild) = message.as_guild() {
		if guild.is_owner() {
			Permission::Admin
		} else if guild.is_admin() {
			Permission::Trusted
		} else {
			Permission::Member
		}
	} else {
		Permission::Member
	}
}

pub(crate) const fn denied_message(required: Permission) -> &'static str {
	match required {
		Permission::SuperAdmin => "暂无权限，需要超级管理员身份",
		Permission::Admin => "暂无权限，需要管理员身份",
		Permission::Trusted => "暂无权限，需要高级用户身份",
		Permission::Member => "暂无权限，需要注册用户身份",
		Permission::User => "暂无权限",
	}
}

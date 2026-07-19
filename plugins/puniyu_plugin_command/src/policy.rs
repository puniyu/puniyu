use puniyu_command::Permission;
use puniyu_event::message::MessageEvent;

pub(crate) fn permission(message: &MessageEvent) -> Permission {
	if let Some(group) = message.as_group() {
		if group.is_owner() {
			Permission::Owner
		} else if group.is_admin() {
			Permission::Admin
		} else {
			Permission::All
		}
	} else if let Some(group) = message.as_group_temp() {
		if group.is_owner() {
			Permission::Owner
		} else if group.is_admin() {
			Permission::Admin
		} else {
			Permission::All
		}
	} else if let Some(guild) = message.as_guild() {
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

pub(crate) const fn denied_message(required: Permission) -> &'static str {
	match required {
		Permission::Master => "暂无权限，只有主人才能操作",
		Permission::Owner => "暂无权限，只有群主或频道主才能操作",
		Permission::Admin => "暂无权限，只有管理员才能操作",
		Permission::All => "暂无权限",
	}
}

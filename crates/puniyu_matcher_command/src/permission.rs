use puniyu_config::Config;
use puniyu_types::event::{EventBase, message::MessageEvent};

/// 检查是否通过黑白名单
/// - 白名单优先：如果白名单不为空，只允许白名单内的 ID
/// - 黑名单：如果在黑名单内，则拒绝
fn check_list(id: &str, enable_list: &[String], disable_list: &[String]) -> bool {
	if !enable_list.is_empty() {
		return enable_list.iter().any(|s| s == id);
	}
	!disable_list.iter().any(|s| s == id)
}

/// 检查消息权限（黑白名单）
pub fn check(event: &MessageEvent) -> bool {
	let config = Config::app();
	match event {
		MessageEvent::Friend(m) => {
			let user_id = m.user_id().to_string();
			let config = config.friend();
			check_list(&user_id, &config.enable_list(), &config.disable_list())
		}
		MessageEvent::Group(m) => {
			let group_id = m.group_id().to_string();
			let config = config.group();
			check_list(&group_id, &config.enable_list(), &config.disable_list())
		}
	}
}

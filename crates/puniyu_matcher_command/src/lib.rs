use puniyu_config::Config;
use puniyu_logger::{debug, info};
use puniyu_registry::command::CommandRegistry;
use puniyu_types::{
	element::RawMessage,
	event::{
		Event, EventBase,
		message::{MessageBase, MessageEvent},
	},
	matcher::Matcher,
};

/// 命令匹配结果
#[derive(Debug, Clone)]
pub struct MatchResult {
	/// 匹配到的命令名称
	pub command_name: String,
	/// 命令参数部分（不包含命令名）
	pub args_text: String,
}

impl MatchResult {
	pub fn new(command_name: impl Into<String>, args_text: impl Into<String>) -> Self {
		Self {
			command_name: command_name.into(),
			args_text: args_text.into(),
		}
	}
}

/// 命令匹配器
pub struct CommandMatcher;

impl CommandMatcher {
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
	fn check_permission(event: &MessageEvent) -> bool {
		match event {
			MessageEvent::Friend(m) => {
				let user_id = m.user_id().to_string();
				let config = Config::app().friend();
				Self::check_list(&user_id, &config.enable_list(), &config.disable_list())
			}
			MessageEvent::Group(m) => {
				let group_id = m.group_id().to_string();
				let config = Config::app().group();
				Self::check_list(&group_id, &config.enable_list(), &config.disable_list())
			}
		}
	}

	/// 记录消息日志
	fn log(event: &MessageEvent) {
		match event {
			MessageEvent::Friend(m) => {
				debug!("收到好友消息: {:?}", m.elements());
				info!(
					"[Bot:{}] [好友消息:{}] {}",
					m.self_id(),
					m.user_id(),
					m.elements().raw()
				);
			}
			MessageEvent::Group(m) => {
				debug!("收到群消息: {:?}", m.elements());
				info!(
					"[Bot:{}] [群消息:{}-{}] {}",
					m.self_id(),
					m.group_id(),
					m.user_id(),
					m.elements().raw()
				);
			}
		}
	}

	/// 从消息中提取纯文本内容
	fn extract_text(event: &MessageEvent) -> String {
		match event {
			MessageEvent::Friend(m) => m
				.elements()
				.iter()
				.filter_map(|e| e.as_text())
				.collect::<Vec<_>>()
				.join(" "),
			MessageEvent::Group(m) => m
				.elements()
				.iter()
				.filter_map(|e| e.as_text())
				.collect::<Vec<_>>()
				.join(" "),
		}
	}

	/// 尝试匹配命令
	fn try_match_command(message_event: &MessageEvent) -> Option<MatchResult> {
		if !Self::check_permission(message_event) {
			return None;
		}

		Self::log(message_event);

		let text = Self::extract_text(message_event).trim().to_string();
		if text.is_empty() {
			return None;
		}

		let global_prefix = Config::app().prefix();
		
		for command in CommandRegistry::get_all() {
			let after_global = if global_prefix.is_empty() {
				text.clone()
			} else if let Some(stripped) = text.strip_prefix(&global_prefix) {
				stripped.to_string()
			} else {
				continue;
			};

			let content = if let Some(plugin_prefix) = command.prefix.as_deref() {
				if plugin_prefix.is_empty() {
					after_global.clone()
				} else if let Some(stripped) = after_global.strip_prefix(plugin_prefix) {
					stripped.to_string()
				} else {
					continue;
				}
			} else {
				after_global.clone()
			};

			let mut parts = content.splitn(2, char::is_whitespace);
			let input_name = match parts.next() {
				Some(n) if !n.trim().is_empty() => n.trim(),
				_ => continue,
			};

			let command_name = command.builder.name();
			let alias = command.builder.alias();
			
			let is_alias_match = alias.is_some_and(|a| a.contains(&input_name));
			if input_name == command_name || is_alias_match {
				let args_text = parts.next().unwrap_or("").trim().to_string();
				return Some(MatchResult::new(command_name, args_text));
			}
		}

		None
	}
}

impl Matcher for CommandMatcher {
	type MatchResult = MatchResult;

	fn name(&self) -> &str {
		"command"
	}

	fn matches(&self, event: &Event) -> Option<Self::MatchResult> {
		if let Event::Message(message_event) = event {
			Self::try_match_command(message_event.as_ref())
		} else {
			None
		}
	}
}

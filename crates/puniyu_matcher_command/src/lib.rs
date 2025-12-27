mod cooldown;
mod message;
mod permission;
mod reactive;

use puniyu_config::Config;
use puniyu_registry::command::CommandRegistry;
use puniyu_types::{
	event::{Event, message::MessageEvent},
	matcher::Matcher,
};

/// 命令匹配结果
#[derive(Debug, Clone)]
pub struct MatchResult {
	/// 匹配到的命令名称
	pub command_name: String,
	/// 命令参数列表
	pub args: Vec<String>,
}

impl MatchResult {
	pub fn new(command_name: impl Into<String>, args: Vec<String>) -> Self {
		Self { command_name: command_name.into(), args }
	}
}

/// 命令匹配器
pub struct CommandMatcher;

impl CommandMatcher {
	/// 尝试匹配命令
	fn try_match_command(event: &MessageEvent) -> Option<MatchResult> {
		if !permission::check(event) {
			return None;
		}

		if cooldown::is_cooling_down(event) {
			return None;
		}

		message::log(event);

		let text = message::extract_text(event).trim().to_string();
		if text.is_empty() {
			return None;
		}

		let (aliases, mode) = reactive::get_bot_config(event);
		let (text, has_alias) = reactive::strip_bot_alias(&text, &aliases);

		if !reactive::check_mode(event, &mode, has_alias) {
			return None;
		}

		if text.is_empty() {
			return None;
		}

		Self::match_commands(&text)
	}

	/// 匹配注册的命令
	fn match_commands(text: &str) -> Option<MatchResult> {
		let config = Config::app();
		let global_prefix = config.prefix();

		for command in CommandRegistry::get_all() {
			let after_global = if global_prefix.is_empty() {
				text.to_string()
			} else if let Some(stripped) = text.strip_prefix(global_prefix) {
				stripped.to_string()
			} else {
				continue;
			};

			let content = match command.prefix.as_deref() {
				Some(prefix) if !prefix.is_empty() => match after_global.strip_prefix(prefix) {
					Some(stripped) => stripped.to_string(),
					None => continue,
				},
				_ => after_global,
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
				let args: Vec<String> =
					parts.next().unwrap_or("").split_whitespace().map(String::from).collect();
				return Some(MatchResult::new(command_name, args));
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

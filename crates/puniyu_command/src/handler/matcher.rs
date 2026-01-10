use crate::{config, cooldown, message, tools};
use puniyu_config::Config;
use puniyu_registry::command::CommandRegistry;
use puniyu_types::event::message::MessageEvent;

pub(crate) struct CommandMatcher<'c> {
	inner: &'c MessageEvent,
}

impl<'c> CommandMatcher<'c> {
	pub fn new(event: &'c MessageEvent) -> Self {
		Self { inner: event }
	}

	pub(crate) fn text(&self) -> String {
		let event = self.inner;
		event.elements().iter().filter_map(|e| e.as_text()).collect::<Vec<_>>().join(" ")
	}

	pub(crate) fn command_name(&self) -> String {
		let event = self.inner;
		let config = Config::app();
		let global_prefix = config.prefix();
		let aliases = config::get_bot_alias(event.self_id());
		let original_text = self.text();
		let text = tools::strip_bot_alias(original_text.as_str(), aliases.as_ref());
		let input_text = text.strip_prefix(global_prefix).unwrap_or(text.as_str());
		input_text.split_whitespace().next().unwrap().trim().to_string()
	}

	pub(crate) fn args(&self) -> Vec<String> {
		let event = self.inner;
		let aliases = config::get_bot_alias(event.self_id());
		let original_text = self.text();
		let text = tools::strip_bot_alias(original_text.as_str(), aliases.as_ref());
		let config = Config::app();
		let global_prefix = config.prefix();

		let input_text = if global_prefix.is_empty() {
			text.as_str()
		} else {
			text.strip_prefix(global_prefix).unwrap_or(&text)
		};

		let mut parts = input_text.splitn(2, char::is_whitespace);
		parts.next();
		let args_str = parts.next().unwrap_or("");
		args_str.split_whitespace().map(String::from).collect()
	}
	pub(crate) fn matcher(&self) -> bool {
		let event = self.inner;
		if !tools::check_perm(event) {
			return false;
		}

		if cooldown::is_cooling_down(event) {
			return false;
		}

		message::log(event);
		let original_text = self.text();

		if original_text.is_empty() {
			return false;
		}
		let bot_id = event.self_id();
		let (aliases, mode) =
			(config::get_bot_alias(bot_id), config::get_bot_reactive_mode(bot_id));

		let text = tools::strip_bot_alias(original_text.as_str(), aliases.as_ref());
		let has_alias = original_text != text;
		if !tools::check_mode(event, &mode, has_alias) {
			return false;
		}
		let config = Config::app();
		let global_prefix = config.prefix();
		let input_text = text.strip_prefix(global_prefix).unwrap_or(&text);
		let command_name = input_text.split_whitespace().next().unwrap_or("").trim();
		if command_name.is_empty() {
			return false;
		}
		CommandRegistry::get_with_name(command_name)
			.map(|cmd| {
				let name_match = command_name == cmd.builder.name();
				let alias_match = cmd.builder.alias().contains(&command_name);
				cooldown::set_cooldown(event);
				name_match || alias_match
			})
			.unwrap_or(false)
	}
}

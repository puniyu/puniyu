use puniyu_command::Command;
use puniyu_command_parser::{CommandParser, Error, ParseResult};
use puniyu_config::{
	OptionConfig, ReactiveMode, app::AppConfig, bot::BotConfig, friend::FriendConfig,
	group::GroupConfig,
};
use puniyu_session::MessageSession;
use puniyu_event::message::MessageEvent;
use std::sync::Arc;

pub(crate) struct CommandInvocation<'m, 'e> {
	pub(crate) message: &'m MessageEvent<'e>,
	pub(crate) parsed: ParseResult,
	pub(crate) candidates: Vec<Arc<dyn Command>>,
	pub(crate) options: OptionConfig,
}

pub(crate) enum ParseOutcome<'m, 'e> {
	NotMatched,
	Invalid(Error),
	Matched(CommandInvocation<'m, 'e>),
}

pub(crate) fn parse<'m, 'e>(
	message: &'m MessageEvent<'e>,
	commands: Vec<Arc<dyn Command>>,
) -> ParseOutcome<'m, 'e> {
	let text = message.get_text().join(" ");
	if text.trim().is_empty() {
		return ParseOutcome::NotMatched;
	}

	let options = resolve_options(message);
	let mut aliases = options.alias.iter().map(ToString::to_string).collect::<Vec<_>>();
	aliases.sort_by_key(|value| std::cmp::Reverse(value.len()));
	aliases.dedup();

	let prefixes = command_prefixes(&commands);
	let has_alias = aliases.iter().any(|alias| !alias.is_empty() && text.starts_with(alias));
	let has_prefix = prefixes.iter().any(|prefix| !prefix.is_empty() && text.starts_with(prefix));
	let mentions_bot = message.get_at().contains(&message.self_id());
	let is_master = MessageSession::new(message, Default::default()).is_master();
	let active = is_active(options.mode, mentions_bot, has_alias, is_master);
	if !active {
		return ParseOutcome::NotMatched;
	}

	let explicit = has_alias || has_prefix || mentions_bot;
	let mut builder = CommandParser::builder();
	for alias in aliases {
		builder = builder.alias(alias);
	}
	for prefix in prefixes {
		builder = builder.prefix(prefix);
	}
	let parsed = match builder.build().parse(&text) {
		Ok(parsed) => parsed,
		Err(error) if explicit => return ParseOutcome::Invalid(error),
		Err(_) => return ParseOutcome::NotMatched,
	};

	let candidates = resolve_candidates(&commands, parsed.command());
	if candidates.is_empty() {
		return if explicit {
			ParseOutcome::Invalid(Error::UnknownCommand { name: parsed.command().to_owned() })
		} else {
			ParseOutcome::NotMatched
		};
	}

	ParseOutcome::Matched(CommandInvocation { message, parsed, candidates, options })
}

fn is_active(mode: ReactiveMode, mentions_bot: bool, has_alias: bool, is_master: bool) -> bool {
	match mode {
		ReactiveMode::All => true,
		ReactiveMode::AtBot => mentions_bot,
		ReactiveMode::Alias => has_alias,
		ReactiveMode::AtOrAlias => mentions_bot || has_alias,
		ReactiveMode::Master => is_master,
	}
}

fn command_prefixes(commands: &[Arc<dyn Command>]) -> Vec<String> {
	let global = AppConfig::get().command().prefix().map(ToOwned::to_owned);
	let mut prefixes = global.iter().cloned().collect::<Vec<_>>();
	prefixes.extend(commands.iter().filter_map(|command| {
		command.prefix().map(|prefix| match &global {
			Some(global) => format!("{global}{prefix}"),
			None => prefix.to_owned(),
		})
	}));
	prefixes.sort_by_key(|value| std::cmp::Reverse(value.len()));
	prefixes.dedup();
	prefixes
}

fn resolve_candidates(commands: &[Arc<dyn Command>], name: &str) -> Vec<Arc<dyn Command>> {
	let mut exact =
		commands.iter().filter(|command| command.name() == name).cloned().collect::<Vec<_>>();
	if !exact.is_empty() {
		exact.sort_by_key(|command| command.priority());
		return exact;
	}

	let mut aliases = commands
		.iter()
		.filter(|command| command.alias().contains(&name))
		.cloned()
		.collect::<Vec<_>>();
	aliases.sort_by_key(|command| command.priority());
	aliases
}

fn resolve_options(message: &MessageEvent<'_>) -> OptionConfig {
	let inherited = BotConfig::get().bot(message.self_id());
	if let Some(group) = message.as_group() {
		GroupConfig::get().resolve(group.group_id(), &inherited)
	} else if let Some(group) = message.as_group_temp() {
		GroupConfig::get().resolve(group.group_id(), &inherited)
	} else if let Some(guild) = message.as_guild() {
		GroupConfig::get().resolve(guild.guild_id(), &inherited)
	} else {
		FriendConfig::get().resolve(message.user_id(), &inherited)
	}
}

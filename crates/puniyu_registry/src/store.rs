#[cfg(feature = "adapter")]
mod adapter;
#[cfg(feature = "bot")]
mod bot;
#[cfg(feature = "command")]
mod command;
#[cfg(feature = "cooldown")]
mod cooldown;
#[cfg(feature = "handler")]
mod handler;
#[cfg(feature = "plugin")]
mod plugin;
#[cfg(feature = "server")]
mod server;

#[cfg(feature = "command")]
use crate::command::Command;
#[cfg(feature = "adapter")]
use puniyu_types::adapter::Adapter;
#[cfg(feature = "bot")]
use puniyu_types::bot::Bot;
#[cfg(feature = "plugin")]
use puniyu_types::plugin::Plugin;
#[cfg(feature = "server")]
use puniyu_types::server::ServerType;
use std::collections::BTreeMap;
#[cfg(any(
	feature = "adapter",
	feature = "bot",
	feature = "command",
	feature = "plugin",
	feature = "server",
	feature = "cooldown"
))]
use std::collections::HashMap;
use std::sync::LazyLock;
#[cfg(any(
	feature = "adapter",
	feature = "bot",
	feature = "command",
	feature = "plugin",
	feature = "server",
	feature = "cooldown"
))]
use std::sync::{Arc, RwLock};

#[cfg(feature = "adapter")]
use adapter::AdapterStore;
#[cfg(feature = "bot")]
use bot::BotStore;
#[cfg(feature = "command")]
use command::CommandStore;
#[cfg(feature = "cooldown")]
use cooldown::CooldownStore;
#[cfg(feature = "handler")]
use handler::HandlerStore;
#[cfg(feature = "plugin")]
use plugin::PluginStore;
#[cfg(feature = "handler")]
use puniyu_types::handler::Handler;
#[cfg(feature = "server")]
use server::ServerStore;

/// 全局 Store 实例
#[allow(dead_code)]
pub(crate) static STORE: LazyLock<Store> = LazyLock::new(Store::default);

#[derive(Default)]
#[allow(dead_code)]
pub(crate) struct Store {
	#[cfg(feature = "adapter")]
	adapter: Arc<RwLock<HashMap<u64, Adapter>>>,
	#[cfg(feature = "bot")]
	bot: Arc<RwLock<HashMap<u64, Bot>>>,
	#[cfg(feature = "command")]
	command: Arc<RwLock<HashMap<u64, Arc<Command>>>>,
	#[cfg(feature = "plugin")]
	plugin: Arc<RwLock<HashMap<u64, Plugin>>>,
	#[cfg(feature = "server")]
	server: Arc<RwLock<HashMap<String, ServerType>>>,
	#[cfg(feature = "cooldown")]
	cooldown: Arc<RwLock<HashMap<String, u64>>>,
	#[cfg(feature = "handler")]
	handler: Arc<RwLock<BTreeMap<u32, Arc<dyn Handler>>>>,
}

impl Store {
	#[allow(dead_code)]
	pub(crate) fn new() -> Self {
		Self::default()
	}

	#[cfg(feature = "adapter")]
	pub(crate) fn adapter(&self) -> AdapterStore {
		AdapterStore(self.adapter.clone())
	}

	#[cfg(feature = "command")]
	pub(crate) fn command(&self) -> CommandStore {
		CommandStore(self.command.clone())
	}

	#[cfg(feature = "plugin")]
	pub(crate) fn plugin(&self) -> PluginStore {
		PluginStore(self.plugin.clone())
	}

	#[cfg(feature = "server")]
	pub(crate) fn server(&self) -> ServerStore {
		ServerStore(self.server.clone())
	}

	#[cfg(feature = "cooldown")]
	pub(crate) fn cooldown(&self) -> CooldownStore {
		CooldownStore(self.cooldown.clone())
	}

	#[cfg(feature = "bot")]
	pub(crate) fn bot(&self) -> BotStore {
		BotStore(self.bot.clone())
	}
	#[cfg(feature = "handler")]
	pub(crate) fn handler(&self) -> HandlerStore {
		HandlerStore(self.handler.clone())
	}
}

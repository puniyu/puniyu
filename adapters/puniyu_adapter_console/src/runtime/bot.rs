use std::sync::Arc;

use super::ConsoleAdapterRuntime;
use puniyu_adapter::{
	account::AccountInfo,
	runtime::{AccountProvider, AdapterRuntime, BotRuntime},
};

#[derive(Debug)]
pub(crate) struct ConsoleBotRuntime {
	adapter: Arc<ConsoleAdapterRuntime>,
	account: AccountInfo,
}

impl ConsoleBotRuntime {
	pub(crate) fn new(adapter: Arc<ConsoleAdapterRuntime>, account: AccountInfo) -> Self {
		Self { adapter, account }
	}
}

impl AccountProvider for ConsoleBotRuntime {
	fn account_info(&self) -> &AccountInfo {
		&self.account
	}
}

impl BotRuntime for ConsoleBotRuntime {
	fn adapter(&self) -> &dyn AdapterRuntime {
		self.adapter.as_ref()
	}
}

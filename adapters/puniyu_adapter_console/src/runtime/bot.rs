use std::sync::Arc;

use async_trait::async_trait;
use super::ConsoleAdapterRuntime;
use puniyu_adapter::prelude::*;

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

impl AdapterProvider for ConsoleBotRuntime {
	fn adapter_info(&self) -> &AdapterInfo {
		self.adapter.adapter_info()
	}
}

impl AccountProvider for ConsoleBotRuntime {
	fn account_info(&self) -> &AccountInfo {
		&self.account
	}
}

#[async_trait]
impl SendMessage for ConsoleBotRuntime {
	async fn send_message(
		&self,
		contact: &ContactType<'_>,
		message: &Message,
	) -> puniyu_adapter::Result<SendMsgType> {
		self.adapter.send_message(contact, message).await
	}
}

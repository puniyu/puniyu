use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use puniyu_adapter::adapter_info;
use puniyu_adapter::contact::ContactType;
use puniyu_adapter::message::Message;
use puniyu_adapter::{
	AdapterCommunication, AdapterInfo, AdapterPlatform, AdapterProtocol, AdapterStandard,
	SendMsgType,
};
use puniyu_adapter::runtime::{AdapterProvider, SendMessage};

use crate::NAME;
use crate::VERSION;
use crate::common::make_random_id;

#[inline]
pub(crate) fn adapter_info() -> AdapterInfo {
	adapter_info!(
		name: NAME,
		version: VERSION,
		platform: AdapterPlatform::Other,
		standard: AdapterStandard::Other,
		protocol: AdapterProtocol::Console,
		communication: AdapterCommunication::Other
	)
}

#[derive(Debug)]
pub struct ConsoleAdapterRuntime {
	adapter: AdapterInfo,
}

impl ConsoleAdapterRuntime {
	pub(crate) fn new() -> Self {
		Self { adapter: adapter_info() }
	}
}

impl AdapterProvider for ConsoleAdapterRuntime {
	fn adapter_info(&self) -> &AdapterInfo {
		&self.adapter
	}
}

#[async_trait]
impl SendMessage for ConsoleAdapterRuntime {
	async fn send_message(
		&self,
		_contact: &ContactType<'_>,
		_message: &Message,
	) -> puniyu_adapter::Result<SendMsgType> {
		let message_id = make_random_id();
		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map_err(Box::<dyn std::error::Error + Send + Sync>::from)?
			.as_secs();

		Ok(SendMsgType { message_id, time: timestamp })
	}
}

mod macros;
mod registry;
use puniyu_adapter_types::{SendMessageOptions, SendMsgResult};
use puniyu_contact::ContactType;
use puniyu_error::AnyError;
use puniyu_message::Message;
pub use registry::BotRegistry;
mod types;
#[doc(inline)]
pub use types::*;
mod error;
pub use error::Error;

use std::{ops::Deref, sync::Arc};

use bytes::Bytes;
use puniyu_adapter_api::AdapterApi;
use smol_str::SmolStr;

#[derive(Clone)]
pub struct Bot {
	api: Arc<dyn AdapterApi>,
	id: SmolStr,
	name: SmolStr,
	avatar: Bytes,
}

impl puniyu_core::bot::Bot for Bot {
	fn id(&self) -> &str {
		self.id.as_str()
	}

	fn name(&self) -> &str {
		self.name.as_str()
	}

	fn avatar(&self) -> Bytes {
		self.avatar.clone()
	}
}

impl Bot {
	pub fn new(adapter: Arc<dyn AdapterApi>) -> Self {
		let account_info = adapter.account_info();

		Self {
			id: account_info.id,
			name: account_info.name,
			avatar: account_info.avatar,
			api: adapter,
		}
	}

	#[inline]
	pub async fn send_message(
		&self,
		contact: &ContactType,
		message: &Message,
		options: Option<&SendMessageOptions>,
	) -> AnyError<SendMsgResult> {
		AdapterApi::send_message(self.api.as_ref(), contact, message, options).await
	}

	pub async fn call_api(
		&self,
		action: &str,
		params: serde_json::Value,
	) -> AnyError<serde_json::Value> {
		AdapterApi::call_api(self.api.as_ref(), action, params).await
	}
}

impl PartialEq for Bot {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id && self.name == other.name && self.avatar == other.avatar
	}
}

impl Eq for Bot {}

impl std::fmt::Debug for Bot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Bot").field("id", &self.id).field("name", &self.name).finish()
	}
}

impl Deref for Bot {
	type Target = Arc<dyn AdapterApi>;

	fn deref(&self) -> &Self::Target {
		&self.api
	}
}

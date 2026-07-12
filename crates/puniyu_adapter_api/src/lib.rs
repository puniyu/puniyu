use async_trait::async_trait;
use log::debug;
use puniyu_account::AccountInfo;
use puniyu_adapter_types::{AdapterInfo, SendMessageOptions, SendMsgResult};
use puniyu_contact::{Contact, ContactType};
use puniyu_error::AnyError;
use puniyu_logger::owo_colors::OwoColorize;
use puniyu_message::Message;

#[async_trait]
pub trait AdapterApi: Send + Sync + SendMessage + CallApi {
	fn adapter_info(&self) -> AdapterInfo;

	fn account_info(&self) -> AccountInfo;

	async fn send_message(
		&self,
		contact: &ContactType,
		message: &Message,
		options: Option<&SendMessageOptions>,
	) -> AnyError<SendMsgResult> {
		let (msg_type, user_id) = match contact {
			ContactType::Friend(friend) => ("PrivateMessage", &friend.peer()),
			ContactType::Group(group) => ("GroupMssage", &group.peer()),
			ContactType::GroupTemp(group) => ("Group TempMessage", &group.peer()),
			ContactType::Guild(guild) => ("GuildMessage", &guild.peer()),
		};
		debug!("[{}:{}]\n{:#?}", format!("Send {}", msg_type).yellow(), user_id.green(), message);
		<Self as SendMessage>::send_message(self, contact, message, options).await
	}

	async fn call_api(
		&self,
		action: &str,
		params: serde_json::Value,
	) -> AnyError<serde_json::Value> {
		<Self as CallApi>::call_api(self, action, params).await
	}
}
#[async_trait]
pub trait SendMessage {
	async fn send_message(
		&self,
		contact: &ContactType,
		message: &Message,
		options: Option<&SendMessageOptions>,
	) -> AnyError<SendMsgResult>;
}

#[async_trait]
pub trait CallApi {
	async fn call_api(
		&self,
		action: &str,
		params: serde_json::Value,
	) -> AnyError<serde_json::Value>;
}

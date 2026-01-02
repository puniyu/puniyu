use crate::common::make_random_id;
use async_trait::async_trait;
use puniyu_adapter::logger::debug;
use puniyu_adapter::prelude::*;
use puniyu_adapter::{AccountApi, FriendApi, MessageApi, Result};
use puniyu_core::Config;
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) static AVATAR_URL: LazyLock<String> = LazyLock::new(|| {
	let config = Config::app();
	let server = config.server();
	format!("http://{}:{}/logo.png", server.host(), server.port())
});
pub struct ConsoleMessageApi;

#[async_trait]
impl MessageApi for ConsoleMessageApi {
	async fn send_msg(&self, contact: ContactType, message: Message) -> Result<SendMsgType> {
		let (msg_type, source) = match &contact {
			ContactType::Friend(friend) => ("私聊消息", &friend.scene),
			ContactType::Group(group) => ("群聊消息", &group.scene),
		};
		let message_id = make_random_id();
		let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

		let elements: Vec<send::Elements> = message.into();

		debug!("[发送{}:{}]\n{:#?}", msg_type, source, elements);

		Ok(SendMsgType { message_id, time: timestamp })
	}
}

pub struct ConsoleAccountApi;

impl AccountApi for ConsoleAccountApi {}

pub struct ConsoleFriendApi;

impl FriendApi for ConsoleFriendApi {}

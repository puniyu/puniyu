use crate::common::make_message_id;
use async_trait::async_trait;
use puniyu_adapter::Result;
use puniyu_adapter::logger::debug;
use puniyu_adapter::prelude::*;
use puniyu_core::Config;
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) static AVATAR_URL: LazyLock<String> = LazyLock::new(|| {
	let server = Config::app().server();
	format!("http://{}:{}/logo.png", server.host(), server.port())
});
pub struct ConsoleAdapterApi;

#[async_trait]
impl AdapterApi for ConsoleAdapterApi {
	async fn get_avatar(&self, _target_id: &str, _size: Option<AvatarSize>) -> Result<Avatar> {
		Ok(AVATAR_URL.clone().into())
	}

	async fn get_group_avatar(&self, _group_id: &str, _size: Option<AvatarSize>) -> Result<Avatar> {
		Ok(AVATAR_URL.clone().into())
	}

	async fn send_msg(&self, contact: ContactType, message: Message) -> Result<SendMsgType> {
		let (msg_type, source) = match &contact {
			ContactType::Friend(friend) => ("私聊消息", &friend.scene),
			ContactType::Group(group) => ("群聊消息", &group.scene),
		};
		let message_id = make_message_id();
		let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

		let elements: Vec<element::send::Elements> = message.into();

		debug!("[发送{}:{}]\n{:#?}", msg_type, source, elements);

		Ok(SendMsgType { message_id, time: timestamp })
	}
}

use crate::common::make_random_id;
use async_trait::async_trait;
use puniyu_adapter_api::contact::ContactType;
use puniyu_adapter_api::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ConsoleMessageApi;

#[async_trait]
impl MessageApi for ConsoleMessageApi {
	async fn send_msg(&self, contact: &ContactType, message: &Message) -> Result<SendMsgType> {
		let (msg_type, source) = match &contact {
			ContactType::Friend(friend) => ("私聊消息", &friend.scene),
			ContactType::Group(group) => ("群聊消息", &group.scene),
		};
		let message_id = make_random_id();
		let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

		debug!("[发送{}:{}]\n{:#?}", msg_type, source, message);

		Ok(SendMsgType { message_id, time: timestamp })
	}
}

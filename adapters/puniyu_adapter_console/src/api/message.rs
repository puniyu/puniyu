use std::time::{SystemTime, UNIX_EPOCH};
use async_trait::async_trait;
use puniyu_adapter::adapter::MessageApi;
use puniyu_adapter::adapter::SendMsgType;
use puniyu_adapter::contact::ContactType;
use puniyu_adapter::element::{Elements, Message};
use puniyu_adapter::logger::debug;
use crate::common::make_random_id;

pub struct ConsoleMessageApi;

#[async_trait]
impl MessageApi for ConsoleMessageApi {
    async fn send_msg(&self, contact: ContactType, message: Message) -> puniyu_adapter::Result<SendMsgType> {
        let (msg_type, source) = match &contact {
            ContactType::Friend(friend) => ("私聊消息", &friend.scene),
            ContactType::Group(group) => ("群聊消息", &group.scene),
        };
        let message_id = make_random_id();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let elements: Vec<Elements> = message.into();

        debug!("[发送{}:{}]\n{:#?}", msg_type, source, elements);

        Ok(SendMsgType { message_id, time: timestamp })
    }
}
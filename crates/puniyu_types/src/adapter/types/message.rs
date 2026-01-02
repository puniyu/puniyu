use serde::Serialize;
use crate::contact::ContactType;
use crate::element::receive::Elements;
use crate::sender::SenderType;

pub enum MessageType {
    MessageId(String),
    MessageSeq(u64),
}

impl From<String> for MessageType {
    fn from(message_id: String) -> Self {
        Self::MessageId(message_id)
    }
}
impl From<&str> for MessageType {
    fn from(message_id: &str) -> Self {
        Self::MessageId(String::from(message_id))
    }
}

impl From<u64> for MessageType {
    fn from(message_seq: u64) -> Self {
        Self::MessageSeq(message_seq)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SendMsgType {
    /// 消息ID
    pub message_id: String,
    /// 发送时间戳(秒)
    pub time: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageInfo {
    /// 消息发送时间戳(秒)
    pub time: u64,
    /// 消息ID
    pub message_id: String,
    /// 消息序列号
    pub message_seq: u64,
    /// 消息联系人
    pub contact: ContactType,
    /// 消息发送者
    pub sender: SenderType,
    /// 消息元素
    pub elements: Vec<Elements>,
}
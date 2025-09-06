use crate::event::{
    contact::FriendContact,
    message::base::{EventParent, EventSub, MessageBase},
    sender::FriendSender,
};
use crate::message::element::receive::ReceiveElements;
use puniyu_utils::adapter::AdapterBase;

pub struct FriendMessageOptions {
    /// 事件id
    pub event_id: String,
    /// 事件父类
    pub event: EventParent,
    /// 事件子类
    pub sub_event: EventSub,
    /// 机器人ID
    pub self_id: String,
    /// 好友ID
    pub user_id: String,
    /// 消息ID
    pub message_id: String,
    /// 消息序列
    pub message_seq: String,
    /// 消息元素
    pub elements: Vec<ReceiveElements>,
    ///  消息文本
    pub msg: Option<String>,
    /// 原始消息
    pub raw_message: Option<String>,
    /// 时间
    pub time: u64,
    /// 好友信息
    pub contact: FriendContact,
    /// 事件发送者
    pub sender: FriendSender,
    /// 机器人
    pub bot: Box<dyn AdapterBase + 'static>,
}
pub struct FriendMessage {
    /// 事件id
    pub event_id: String,
    /// 事件父类
    pub event: String,
    /// 事件子类
    pub sub_event: String,
    /// 机器人ID
    pub self_id: String,
    /// 好友ID
    pub user_id: String,
    /// 消息ID
    pub message_id: String,
    /// 消息序列
    pub message_seq: String,
    /// 消息元素
    pub elements: Vec<ReceiveElements>,
    /// 消息文本
    pub msg: Option<String>,
    /// 原始消息
    pub raw_message: Option<String>,
    /// 时间
    pub time: u64,
    /// 好友信息
    pub contact: FriendContact,
    /// 事件发送者
    pub sender: FriendSender,
    /// 机器人
    pub bot: Box<dyn AdapterBase>,
}

impl FriendMessage {
    /// 这里都是接受的消息段
    pub fn new(data: FriendMessageOptions) -> Self {
        Self {
            event_id: data.event_id,
            event: data.event.to_parent_string(),
            sub_event: data.sub_event.to_sub_string(),
            self_id: data.self_id,
            user_id: data.user_id,
            message_id: data.message_id,
            message_seq: data.message_seq,
            time: data.time,
            contact: data.contact,
            sender: data.sender,
            bot: data.bot,
            elements: data.elements,
            msg: data.msg,
            raw_message: data.raw_message,
        }
    }

    fn get_sender(&self) -> &FriendSender {
        &self.sender
    }
    fn get_is_private(&self) -> bool {
        true
    }

    fn get_is_friend(&self) -> bool {
        true
    }
}

impl MessageBase for FriendMessage {
    fn event_id(&self) -> &str {
        &self.event_id
    }
    fn event(&self) -> &str {
        self.event.as_str()
    }

    fn sub_event(&self) -> &str {
        self.sub_event.as_str()
    }

    fn self_id(&self) -> &str {
        &self.self_id
    }

    fn elements(&self) -> &Vec<ReceiveElements> {
        &self.elements
    }

    fn msg(&self) -> Option<&str> {
        self.msg.as_deref()
    }

    fn raw_message(&self) -> Option<&str> {
        self.raw_message.as_deref()
    }

    fn get_message_id(&self) -> &str {
        &self.message_id
    }
}

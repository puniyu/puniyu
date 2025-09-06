use crate::message::element::receive::ReceiveElements;

pub enum EventParent {
    Message,
    Notice,
    Request,
}

impl EventParent {
    pub fn to_str(&self) -> &'static str {
        match self {
            EventParent::Message => "message",
            EventParent::Notice => "notice",
            EventParent::Request => "request",
        }
    }

    pub fn to_parent_string(&self) -> String {
        match self {
            EventParent::Message => "message".to_string(),
            EventParent::Notice => "notice".to_string(),
            EventParent::Request => "request".to_string(),
        }
    }
}

pub enum EventSub {
    Friend,
    Group,
    Guild,
}

impl EventSub {
    pub fn to_str(&self) -> &'static str {
        match self {
            EventSub::Friend => "friend",
            EventSub::Group => "group",
            EventSub::Guild => "guild",
        }
    }

    pub fn to_sub_string(&self) -> String {
        match self {
            EventSub::Friend => "friend".to_string(),
            EventSub::Group => "group".to_string(),
            EventSub::Guild => "guild".to_string(),
        }
    }
}

pub trait MessageBase {
    fn event_id(&self) -> &str;
    fn event(&self) -> &str;

    /// 获取子事件类型
    fn sub_event(&self) -> &str;

    fn self_id(&self) -> &str;

    /// 获取消息元素
    fn elements(&self) -> &Vec<ReceiveElements>;

    /// 获取消息文本
    fn msg(&self) -> Option<&str>;

    /// 获取原始消息
    fn raw_message(&self) -> Option<&str>;

    /// 获取 self_id
    fn get_message_id(&self) -> &str;

    fn get_at(&self) -> Vec<String> {
        self.elements()
            .iter()
            .filter_map(|e| match e {
                ReceiveElements::At(at) => Some(at.target_id.to_string()),
                _ => None,
            })
            .collect()
    }

    fn get_all(&self) -> bool {
        self.elements()
            .iter()
            .any(|e| matches!(e, ReceiveElements::At(at) if at.is_all()))
    }
    fn get_at_bot(&self) -> bool {
        self.elements()
            .iter()
            .any(|e| matches!(e, ReceiveElements::At(at) if at.target_id.contains(self.self_id())))
    }

    fn get_at_online(&self) -> bool {
        self.elements()
            .iter()
            .any(|e| matches!(e, ReceiveElements::At(at) if at.is_online()))
    }

    /// 获取图片元素
    fn get_image(&self) -> Vec<String> {
        self.elements()
            .iter()
            .filter_map(|e| match e {
                ReceiveElements::Image(image) => Some(image.file.to_string()),
                _ => None,
            })
            .collect()
    }

    fn get_record(&self) -> Vec<String> {
        self.elements()
            .iter()
            .filter_map(|e| match e {
                ReceiveElements::Record(record) => Some(record.file.to_string()),
                _ => None,
            })
            .collect()
    }

    fn get_reply_id(&self) -> Option<String> {
        self.elements()
            .iter()
            .filter_map(|e| match e {
                ReceiveElements::Reply(reply) => Some(reply.message_id.to_string()),
                _ => None,
            })
            .next()
    }
}

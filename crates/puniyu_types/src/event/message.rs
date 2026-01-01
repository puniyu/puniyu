use super::EventBase;
use crate::bot::BotInfo;
use crate::contact::{FriendContact, GroupContact, Scene};
use crate::element::receive::Elements;
use crate::event::EventType;
use crate::sender::{FriendSender, GroupSender, Role};
use bytes::Bytes;
use puniyu_config::Config;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, EnumString, Display, IntoStaticStr)]
pub enum MessageSubType {
	#[strum(serialize = "friend")]
	Friend,
	#[strum(serialize = "group")]
	Group,
	#[strum(serialize = "guild")]
	Guild,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0")]
pub enum MessageEvent {
	Friend(FriendMessage),
	Group(GroupMessage),
}

impl MessageEvent {
	pub fn is_friend(&self) -> bool {
		match self {
			MessageEvent::Friend(message) => message.is_friend(),
			MessageEvent::Group(message) => message.is_friend(),
		}
	}

	pub fn is_group(&self) -> bool {
		match self {
			MessageEvent::Friend(message) => message.is_group(),
			MessageEvent::Group(message) => message.is_group(),
		}
	}

	pub fn as_friend(&self) -> Option<&FriendMessage> {
		match self {
			MessageEvent::Friend(msg) => Some(msg),
			_ => None,
		}
	}
	pub fn as_group(&self) -> Option<&GroupMessage> {
		match self {
			MessageEvent::Group(msg) => Some(msg),
			_ => None,
		}
	}

	pub fn get_bot(&self) -> &BotInfo {
		match self {
			MessageEvent::Friend(msg) => msg.bot(),
			MessageEvent::Group(msg) => msg.bot(),
		}
	}

	pub fn time(&self) -> u64 {
		match self {
			MessageEvent::Friend(msg) => msg.time(),
			MessageEvent::Group(msg) => msg.time(),
		}
	}

	pub fn event(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.event(),
			MessageEvent::Group(msg) => msg.event(),
		}
	}

	pub fn event_id(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.event_id(),
			MessageEvent::Group(msg) => msg.event_id(),
		}
	}

	pub fn sub_event(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.sub_event(),
			MessageEvent::Group(msg) => msg.sub_event(),
		}
	}

	pub fn self_id(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.self_id(),
			MessageEvent::Group(msg) => msg.self_id(),
		}
	}

	pub fn user_id(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.user_id(),
			MessageEvent::Group(msg) => msg.user_id(),
		}
	}

	pub fn message_id(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.message_id(),
			MessageEvent::Group(msg) => msg.message_id(),
		}
	}

	pub fn elements(&self) -> Vec<Elements> {
		match self {
			MessageEvent::Friend(msg) => msg.elements(),
			MessageEvent::Group(msg) => msg.elements(),
		}
	}

	pub fn get_text(&self) -> String {
		match self {
			MessageEvent::Friend(msg) => msg.get_text(),
			MessageEvent::Group(msg) => msg.get_text(),
		}
	}

	pub fn get_at(&self) -> Option<Vec<String>> {
		match self {
			MessageEvent::Friend(msg) => msg.get_at(),
			MessageEvent::Group(msg) => msg.get_at(),
		}
	}

	pub fn get_image(&self) -> Option<Bytes> {
		match self {
			MessageEvent::Friend(msg) => msg.get_image(),
			MessageEvent::Group(msg) => msg.get_image(),
		}
	}

	pub fn get_record(&self) -> Option<Bytes> {
		match self {
			MessageEvent::Friend(msg) => msg.get_record(),
			MessageEvent::Group(msg) => msg.get_record(),
		}
	}

	pub fn get_reply_id(&self) -> Option<String> {
		match self {
			MessageEvent::Friend(msg) => msg.get_reply_id(),
			MessageEvent::Group(msg) => msg.get_reply_id(),
		}
	}

	pub fn group_id(&self) -> Option<&str> {
		match self {
			MessageEvent::Group(msg) => Some(msg.group_id()),
			_ => None,
		}
	}

	pub fn is_master(&self) -> bool {
		match self {
			MessageEvent::Friend(msg) => msg.is_master(),
			MessageEvent::Group(msg) => msg.is_master(),
		}
	}
}

impl fmt::Display for MessageEvent {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			MessageEvent::Friend(msg) => write!(f, "{}", msg),
			MessageEvent::Group(msg) => write!(f, "{}", msg),
		}
	}
}

pub trait MessageBase: Send + Sync + EventBase {
	/// message_id
	fn message_id(&self) -> &str;
	/// 消息元素
	fn elements(&self) -> Vec<Elements>;

	/// 获取文本元素
	fn get_text(&self) -> String {
		self.elements()
			.into_iter()
			.filter_map(|e| match e {
				Elements::Text(text) => Some(text.text.clone()),
				_ => None,
			})
			.collect::<Vec<String>>()
			.join("")
	}

	/// 获取艾特元素
	fn get_at(&self) -> Option<Vec<String>> {
		let at_list = self
			.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::At(at) => Some(at.target_id.to_string()),
				_ => None,
			})
			.collect::<Vec<String>>();
		if at_list.is_empty() { None } else { Some(at_list) }
	}

	/// 获取图片元素
	fn get_image(&self) -> Option<Bytes> {
		self.elements()
			.into_iter()
			.filter_map(|e| match e {
				Elements::Image(image) => Some(image.file),
				_ => None,
			})
			.next()
	}

	/// 获取语音元素
	fn get_record(&self) -> Option<Bytes> {
		self.elements()
			.into_iter()
			.filter_map(|e| match e {
				Elements::Record(record) => Some(record.file),
				_ => None,
			})
			.next()
	}

	/// 获取回复Id
	fn get_reply_id(&self) -> Option<String> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::Reply(reply) => Some(reply.message_id.clone()),
				_ => None,
			})
			.next()
	}

	fn is_master(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct MessageBuilder<Contact, Sender> {
	pub bot: BotInfo,
	pub event_id: String,
	pub self_id: String,
	pub user_id: String,
	pub contact: Contact,
	pub sender: Sender,
	pub time: u64,
	pub message_id: String,
	pub elements: Vec<Elements>,
}

fn serialize_arc_bot_info<S>(bot_info: &Arc<BotInfo>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: serde::Serializer,
{
	bot_info.serialize(serializer)
}

fn deserialize_arc_bot_info<'de, D>(deserializer: D) -> Result<Arc<BotInfo>, D::Error>
where
	D: serde::Deserializer<'de>,
{
	let bot_info = BotInfo::deserialize(deserializer)?;
	Ok(Arc::new(bot_info))
}

macro_rules! impl_message_event {
    (
        $(
            $struct_name:ident, $contact_ty:ty, $sender_ty:ty, $sub_event:expr;
        )+
    ) => {
        $(
            impl_message_event!(@impl $struct_name, $contact_ty, $sender_ty, $sub_event);
        )+
    };

    (
        $struct_name:ident, $contact_ty:ty, $sender_ty:ty, $sub_event:expr
    ) => {
        impl_message_event!(@impl $struct_name, $contact_ty, $sender_ty, $sub_event);
    };

    (@impl $struct_name:ident, $contact_ty:ty, $sender_ty:ty, $sub_event:expr) => {

		#[derive(Debug, Clone, Deserialize, Serialize)]
        pub struct $struct_name {
            #[serde(
				serialize_with = "serialize_arc_bot_info",
				deserialize_with = "deserialize_arc_bot_info"
			)]
            bot: Arc<BotInfo>,
            event_id: String,
            time: u64,
            self_id: String,
            user_id: String,
			message_id: String,
			elements: Vec<Elements>,
            contact: $contact_ty,
            sender: $sender_ty,
        }

		impl $struct_name {
            pub fn new(message_builder: MessageBuilder<$contact_ty, $sender_ty>) -> Self {
                Self {
                    bot: Arc::new(message_builder.bot),
                    event_id: message_builder.event_id,
                    time: message_builder.time,
                    self_id: message_builder.self_id,
                    user_id: message_builder.user_id,
                    message_id: message_builder.message_id,
                    elements: message_builder.elements,
                    contact: message_builder.contact,
                    sender: message_builder.sender,
                }
            }
        }
        impl EventBase for $struct_name {
            type ContactType = $contact_ty;
            type SenderType = $sender_ty;

			fn bot(&self) -> &BotInfo {
                &self.bot
            }

            fn time(&self) -> u64 {
                self.time
            }

            fn event(&self) -> &str {
                EventType::Message.into()
            }

            fn event_id(&self) -> &str {
                &self.event_id
            }

            fn sub_event(&self) -> &str {
                $sub_event.into()
            }

            fn self_id(&self) -> &str {
                &self.self_id
            }

            fn user_id(&self) -> &str {
                &self.user_id
            }

            fn contact(&self) -> Self::ContactType {
                self.contact.clone()
            }

            fn sender(&self) -> Self::SenderType {
                self.sender.clone()
            }

            fn is_friend(&self) -> bool {
                matches!(self.contact().scene, Scene::Friend)
            }

            fn is_group(&self) -> bool {
                matches!(self.contact().scene, Scene::Group)
            }
        }

        impl MessageBase for $struct_name {
            fn message_id(&self) -> &str {
                &self.message_id
            }

            fn elements(&self) -> Vec<Elements> {
                self.elements.clone()
            }

            fn is_master(&self) -> bool {
				let config = Config::app();
                let masters = config.masters();
                masters.contains(&self.user_id)
            }
        }

        impl fmt::Display for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("event_id", &self.event_id)
                    .field("self_id", &self.self_id)
                    .field("user_id", &self.user_id)
                    .field("message_id", &self.message_id)
                    .field("elements", &self.elements)
                    .finish()
            }
        }
    };
}

impl_message_event!(
	GroupMessage, GroupContact, GroupSender, MessageSubType::Group;
	FriendMessage, FriendContact, FriendSender, MessageSubType::Friend;
);

impl GroupMessage {
	pub fn group_id(&self) -> &str {
		&self.contact.peer
	}

	pub fn is_admin(&self) -> bool {
		matches!(self.sender.role, Role::Admin)
	}

	pub fn is_owner(&self) -> bool {
		matches!(self.sender.role, Role::Owner)
	}
}

#[cfg(feature = "event")]
#[macro_export]
macro_rules! create_message_event {
    (
        Group,
        $( $key:ident : $value:expr ),* $(,)?
    ) => {{
        let mut builder = $crate::event::message::MessageBuilder::<$crate::contact::GroupContact, $crate::sender::GroupSender> {
            bot: Default::default(),
            event_id: String::new(),
            time: 0,
            self_id: String::new(),
            user_id: String::new(),
            message_id: String::new(),
            elements: vec![],
            contact: Default::default(),
            sender: Default::default(),
        };

        $(
            builder.$key = create_message_event!(@convert $key, $value);
        )*

        let message = $crate::event::message::GroupMessage::new(builder);
        let event = $crate::event::Event::Message(Box::new($crate::event::message::MessageEvent::Group(message)));
     	$crate::send_event!(event);
    }};

    (
        Friend,
        $( $key:ident : $value:expr ),* $(,)?
    ) => {{
        let mut builder = $crate::event::message::MessageBuilder::<$crate::contact::FriendContact, $crate::sender::FriendSender> {
            bot: Default::default(),
            event_id: String::new(),
            time: 0,
            self_id: String::new(),
            user_id: String::new(),
            message_id: String::new(),
            elements: vec![],
            contact: Default::default(),
            sender: Default::default(),
        };

        $(
            builder.$key = create_message_event!(@convert $key, $value);
        )*

        let message = $crate::event::message::FriendMessage::new(builder);
        let event = $crate::event::Event::Message(Box::new($crate::event::message::MessageEvent::Friend(message)));
     	$crate::send_event!(event);
    }};

    (@convert bot, $v:expr) => { std::sync::Arc::clone(&$v.clone()).into() };
    (@convert event_id, $v:expr) => { $v.to_string() };
    (@convert contact, $v:expr) => { $v };
    (@convert self_id, $v:expr) => { $v.to_string() };
    (@convert user_id, $v:expr) => { $v.to_string() };
    (@convert message_id, $v:expr) => { $v.to_string() };
    (@convert elements, $v:expr) => { $v };
    (@convert sender, $v:expr) => { $v };
    (@convert time, $v:expr) => { $v };
}

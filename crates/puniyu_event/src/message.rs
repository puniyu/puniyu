mod friend;
use std::sync::Arc;

#[doc(inline)]
pub use friend::FriendMessage;
mod group;
#[doc(inline)]
pub use group::{GroupMessage, GroupTempMessage};
mod guild;
#[doc(inline)]
pub use guild::GuildMessage;

use ecow::EcoVec;
use puniyu_bot::Bot;
use puniyu_contact::ContactType;
use puniyu_element::receive::Elements;
use puniyu_sender::SenderType;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};
use crate::EventBase;


#[derive(
	Debug,
	Clone,
	Hash,
	Copy,
	PartialEq,
	Eq,
	EnumString,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SubEventType {
	Friend,
	Group,
	GroupTemp,
	Guild,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageEvent {
	Friend(FriendMessage),
	Group(GroupMessage),
	GroupTemp(GroupTempMessage),
	Guild(GuildMessage),
}

impl MessageEvent {
	pub fn as_friend(&self) -> Option<&FriendMessage> {
		match self {
			MessageEvent::Friend(e) => Some(e),
			_ => None,
		}
	}
	pub fn as_group(&self) -> Option<&GroupMessage> {
		match self {
			MessageEvent::Group(e) => Some(e),
			_ => None,
		}
	}
	pub fn as_group_temp(&self) -> Option<&GroupTempMessage> {
		match self {
			MessageEvent::GroupTemp(e) => Some(e),
			_ => None,
		}
	}
	pub fn as_guild(&self) -> Option<&GuildMessage> {
		match self {
			MessageEvent::Guild(e) => Some(e),
			_ => None,
		}
	}
}

macro_rules! forward_event {
	($name:ident -> $ret:ty, |$m:ident| $body:expr) => {
		impl MessageEvent {
			pub fn $name(&self) -> $ret {
				match self {
					Self::Friend($m) => $body,
					Self::Group($m) => $body,
					Self::GroupTemp($m) => $body,
					Self::Guild($m) => $body,
				}
			}
		}
	};
}

forward_event!(time -> u64, |m| m.time());
forward_event!(event_id -> &str, |m| m.event_id());
forward_event!(bot -> Arc<dyn Bot>, |m| m.bot());
forward_event!(self_id -> &str, |m| m.self_id());
forward_event!(user_id -> &str, |m| m.user_id());
forward_event!(message_id -> &str, |m| m.message_id());
forward_event!(elements -> &EcoVec<Elements>, |m| m.elements());
forward_event!(get_text -> Vec<&str>, |m| m.get_text());
forward_event!(get_at -> Vec<&str>, |m| m.get_at());
forward_event!(get_reply_id -> Option<&str>, |m| m.get_reply_id());

impl MessageEvent {
	pub fn event_type(&self) -> crate::EventType {
		crate::EventType::Message
	}
	pub fn sub_event(&self) -> SubEventType {
		match self {
			Self::Friend(m) => m.sub_event(),
			Self::Group(m) => m.sub_event(),
			Self::GroupTemp(m) => m.sub_event(),
			Self::Guild(m) => m.sub_event(),
		}
	}
	pub fn contact(&self) -> ContactType {
		match self {
			Self::Friend(m) => ContactType::Friend(m.contact()),
			Self::Group(m) => ContactType::Group(m.contact()),
			Self::GroupTemp(m) => ContactType::GroupTemp(m.contact()),
			Self::Guild(m) => ContactType::Guild(m.contact()),
		}
	}
	pub fn sender(&self) -> SenderType {
		match self {
			Self::Friend(m) => SenderType::Friend(m.sender()),
			Self::Group(m) => SenderType::Group(m.sender()),
			Self::GroupTemp(m) => SenderType::GroupTemp(m.sender()),
			Self::Guild(m) => SenderType::Guild(m.sender()),
		}
	}
}

pub trait MessageBase: EventBase {
	fn message_id(&self) -> &str;
	fn elements(&self) -> &EcoVec<Elements>;

	fn get_text(&self) -> Vec<&str> {
		self.elements()
			.iter()
			.filter_map(|element| match element {
				Elements::Text(text) => Some(text.text.as_str()),
				_ => None,
			})
			.collect()
	}

	fn get_at(&self) -> Vec<&str> {
		self.elements()
			.iter()
			.filter_map(|element| match element {
				Elements::At(at) => Some(at.target_id.as_str()),
				_ => None,
			})
			.collect()
	}

	fn get_reply_id(&self) -> Option<&str> {
		self.elements().iter().find_map(|element| match element {
			Elements::Reply(reply) => Some(reply.message_id.as_str()),
			_ => None,
		})
	}
}

macro_rules! impl_message {
	(
		$name:ident,
		$event_type:expr,
		$sub_event:expr,
		$contact:ty,
		$sender:ty
	) => {
		impl crate::EventBase for $name {
			type Contact = $contact;
			type Sender = $sender;
			type EventType = crate::EventType;
			type SubEventType = super::SubEventType;

			fn time(&self) -> u64 {
				self.time
			}
			fn event_type(&self) -> Self::EventType {
				$event_type
			}
			fn event_id(&self) -> &str {
				&self.event_id
			}
			fn sub_event(&self) -> Self::SubEventType {
				$sub_event
			}
			fn bot(&self) -> std::sync::Arc<dyn puniyu_bot::Bot> {
				self.bot.clone()
			}
			fn self_id(&self) -> &str {
				self.bot.id()
			}
			fn user_id(&self) -> &str {
				self.sender.user_id()
			}
			fn contact(&self) -> $contact {
				self.contact.clone()
			}
			fn sender(&self) -> $sender {
				self.sender.clone()
			}
		}

		impl $crate::message::MessageBase for $name {
			fn message_id(&self) -> &str {
				&self.message_id
			}
			fn elements(&self) -> &ecow::EcoVec<puniyu_element::receive::Elements> {
				&self.elements
			}
		}
	};
}

pub(crate) use impl_message;

#[cfg(test)]
mod tests {
	use super::*;
	use puniyu_contact::FriendContact;
	use puniyu_element::receive::{AtElement, ReplyElement, TextElement};
	use puniyu_sender::FriendSender;

	#[derive(Debug, PartialEq, Eq)]
	struct TestMessage {
		elements: EcoVec<Elements>,
	}

	impl EventBase for TestMessage {
		type Contact = FriendContact;
		type Sender = FriendSender;
		type EventType = ();
		type SubEventType = ();

		fn time(&self) -> u64 {
			0
		}

		fn event_type(&self) -> Self::EventType {
			
		}

		fn event_id(&self) -> &str {
			"event"
		}

		fn sub_event(&self) -> Self::SubEventType {
			
		}

		fn bot(&self) -> Arc<dyn Bot> {
			unreachable!("访问器测试不需要机器人实例")
		}

		fn self_id(&self) -> &str {
			"bot"
		}

		fn user_id(&self) -> &str {
			"user"
		}

		fn contact(&self) -> FriendContact {
			unreachable!("访问器测试不需要联系人实例")
		}

		fn sender(&self) -> FriendSender {
			unreachable!("访问器测试不需要发送者实例")
		}
	}

	impl MessageBase for TestMessage {
		fn message_id(&self) -> &str {
			"message"
		}

		fn elements(&self) -> &EcoVec<Elements> {
			&self.elements
		}
	}

	#[test]
	fn test_message_element_accessors_return_str_slices() {
		let message = TestMessage {
			elements: [
				Elements::from(TextElement::from("第一段")),
				Elements::from(AtElement::from("user-1")),
				Elements::from(TextElement::from("第二段")),
				Elements::from(ReplyElement::from("message-1")),
			]
			.into_iter()
			.collect(),
		};

		assert_eq!(message.get_text(), vec!["第一段", "第二段"]);
		assert_eq!(message.get_at(), vec!["user-1"]);
		assert_eq!(message.get_reply_id(), Some("message-1"));
	}
}

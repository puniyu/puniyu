mod friend;
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
use puniyu_core::event::EventBase;
use puniyu_element::receive::Elements;
use puniyu_sender::SenderType;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

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
	($name:ident -> $ret:ty) => {
		impl MessageEvent {
			pub fn $name(&self) -> $ret {
				match self {
					Self::Friend(m) => m.$name(),
					Self::Group(m) => m.$name(),
					Self::GroupTemp(m) => m.$name(),
					Self::Guild(m) => m.$name(),
				}
			}
		}
	};
}

forward_event!(time -> u64);
forward_event!(event_type -> crate::EventType);
forward_event!(event_id -> &str);
forward_event!(sub_event -> SubEventType);
forward_event!(bot -> &Bot);
forward_event!(self_id -> &str);
forward_event!(user_id -> &str);
impl MessageEvent {
	pub fn contact(&self) -> ContactType {
		match self {
			Self::Friend(m) => ContactType::Friend(m.contact()),
			Self::Group(m) => ContactType::Group(m.contact()),
			Self::GroupTemp(m) => ContactType::GroupTemp(m.contact()),
			Self::Guild(m) => ContactType::Guild(m.contact()),
		}
	}
}
impl MessageEvent {
	pub fn sender(&self) -> SenderType {
		match self {
			Self::Friend(m) => SenderType::Friend(m.sender()),
			Self::Group(m) => SenderType::Group(m.sender()),
			Self::GroupTemp(m) => SenderType::GroupTemp(m.sender()),
			Self::Guild(m) => SenderType::Guild(m.sender()),
		}
	}
}
forward_event!(message_id -> &str);
forward_event!(elements -> &EcoVec<Elements>);
forward_event!(get_text -> Vec<&str>);
forward_event!(get_at -> Vec<&str>);
forward_event!(get_reply_id -> Option<&str>);

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
		$contact:ty,
		$sender:ty,
		$sub_event:expr
	) => {
		impl puniyu_core::event::EventBase for $name {
			type Bot = puniyu_bot::Bot;
			type Contact = $contact;
			type Sender = $sender;
			type EventType = $crate::EventType;
			type SubEventType = super::SubEventType;

			fn time(&self) -> u64 {
				self.time
			}
			fn event_type(&self) -> $crate::EventType {
				$crate::EventType::Message
			}
			fn event_id(&self) -> &str {
				&self.event_id
			}
			fn sub_event(&self) -> $crate::message::SubEventType {
				$sub_event
			}
			fn bot(&self) -> &puniyu_bot::Bot {
				&self.bot
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
		type Bot = Bot;
		type Contact = FriendContact;
		type Sender = FriendSender;
		type EventType = ();
		type SubEventType = ();

		fn time(&self) -> u64 {
			0
		}

		fn event_type(&self) {}

		fn event_id(&self) -> &str {
			"event"
		}

		fn sub_event(&self) {}

		fn bot(&self) -> &Bot {
			unreachable!("访问器测试不需要机器人实例")
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

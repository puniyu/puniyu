use super::{MessageBase, MessageBuilder, MessageSubEventType};
use crate::{EventBase, EventType};
use puniyu_bot::Bot;
use puniyu_contact::FriendContact as Contact;
use puniyu_element::receive::Elements;
use puniyu_sender::FriendSender as Sender;

/// 好友消息事件
///
/// 表示从好友接收到的消息事件。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_event::message::{FriendMessage, MessageBase};
///
/// fn handle_friend_message(msg: &FriendMessage) {
///     // 获取消息文本
///     let texts = msg.get_text();
///     println!("收到好友消息: {:?}", texts);
///     
///     // 获取发送者信息
///     let sender = msg.sender();
///     println!("发送者: {}", sender.user_id());
/// }
/// ```
#[derive(Debug, Clone)]
pub struct FriendMessage<'m> {
	bot: &'m Bot,
	event_id: &'m str,
	time: u64,
	user_id: &'m str,
	message_id: &'m str,
	elements: &'m Vec<Elements<'m>>,
	contact: &'m Contact<'m>,
	sender: &'m Sender<'m>,
}

impl<'m> FriendMessage<'m> {
	/// 从消息构建器创建好友消息
	///
	/// # 参数
	///
	/// - `builder`: 消息构建器
	pub fn new(builder: MessageBuilder<'m, Contact, Sender>) -> Self {
		Self {
			bot: builder.bot,
			event_id: builder.event_id,
			time: builder.time,
			user_id: builder.user_id,
			message_id: builder.message_id,
			elements: builder.elements,
			contact: builder.contact,
			sender: builder.sender,
		}
	}
}

impl<'e> EventBase for FriendMessage<'e> {
	type EventType = EventType;
	type SubEventType = MessageSubEventType;
	type Contact = Contact<'e>;
	type Sender = Sender<'e>;

	fn time(&self) -> u64 {
		self.time
	}

	fn event(&self) -> &EventType {
		&EventType::Message
	}

	fn event_id(&self) -> &str {
		self.event_id
	}

	fn sub_event(&self) -> &MessageSubEventType {
		&MessageSubEventType::Group
	}

	fn bot(&self) -> &Bot {
		self.bot
	}

	fn self_id(&self) -> &str {
		self.bot.account().uin.as_str()
	}

	fn user_id(&self) -> &str {
		self.user_id
	}

	fn contact(&self) -> &Self::Contact {
		self.contact
	}

	fn sender(&self) -> &Self::Sender {
		self.sender
	}
}

impl<'m> MessageBase for FriendMessage<'m> {
	fn message_id(&self) -> &str {
		self.message_id
	}

	fn elements(&self) -> &Vec<Elements<'_>> {
		self.elements
	}
}

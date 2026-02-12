use puniyu_bot::Bot;
use super::{MessageBase, MessageBuilder, MessageSubType};
use crate::{EventBase, EventType};
use puniyu_contact::GroupContact as Contact;
use puniyu_element::receive::Elements;
use puniyu_sender::GroupSender as Sender;
use puniyu_sender::Role;

/// 群消息事件
///
/// 表示从群聊接收到的消息事件。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_event::message::{GroupMessage, MessageBase};
///
/// fn handle_group_message(msg: &GroupMessage) {
///     // 获取群 ID
///     let group_id = msg.group_id();
///     
///     // 判断发送者是否为管理员
///     if msg.is_admin() {
///         println!("管理员发送的消息");
///     }
///     
///     // 获取消息文本
///     let texts = msg.get_text();
///     println!("群消息: {:?}", texts);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct GroupMessage<'m> {
	bot: &'m Bot,
	event_id: &'m str,
	time: u64,
	self_id: &'m str,
	user_id: &'m str,
	message_id: &'m str,
	elements: &'m Vec<Elements<'m>>,
	contact: &'m Contact<'m>,
	sender: &'m Sender<'m>,
}

impl<'m> GroupMessage<'m> {
	/// 从消息构建器创建群消息
	///
	/// # 参数
	///
	/// - `builder`: 消息构建器
	pub fn new(builder: MessageBuilder<'m, Contact, Sender>) -> Self {
		Self {
			bot: builder.bot,
			event_id: builder.event_id,
			time: builder.time,
			self_id: builder.self_id,
			user_id: builder.user_id,
			message_id: builder.message_id,
			elements: builder.elements,
			contact: builder.contact,
			sender: builder.sender,
		}
	}

	/// 获取群 ID
	///
	/// # 返回值
	///
	/// 返回群组的唯一标识符
	pub fn group_id(&self) -> &str {
		self.contact.peer
	}

	/// 判断发送者是否为管理员
	///
	/// # 返回值
	///
	/// 如果发送者是管理员返回 `true`，否则返回 `false`
	pub fn is_admin(&self) -> bool {
		matches!(self.sender.role, Role::Admin)
	}

	/// 判断发送者是否为群主
	///
	/// # 返回值
	///
	/// 如果发送者是群主返回 `true`，否则返回 `false`
	pub fn is_owner(&self) -> bool {
		matches!(self.sender.role, Role::Owner)
	}
}

impl<'e> EventBase for GroupMessage<'e> {
	type EventType = EventType;
	type SubEventType = MessageSubType;
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

	fn sub_event(&self) -> &MessageSubType {
		&MessageSubType::Group
	}

	fn bot(&self) -> &Bot {
		self.bot
	}

	fn self_id(&self) -> &str {
		self.self_id
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

impl<'m> MessageBase for GroupMessage<'m> {
	fn message_id(&self) -> &str {
		self.message_id
	}

	fn elements(&self) -> &Vec<Elements<'_>> {
		self.elements
	}
}

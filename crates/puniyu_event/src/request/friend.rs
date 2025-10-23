use crate::request::{RequestBase, RequestBuilder, RequestSubEvent};
use crate::{EventBase, EventType};
use puniyu_contact::FriendContact;
use puniyu_sender::FriendSender;

#[derive(Debug, Clone)]
pub struct PrivateApplyType {
	/// 验证信息
	pub message: String,
}

#[derive(Debug, Clone)]
pub struct PrivateApply {
	/// 事件id
	event_id: String,
	/// 时间戳
	time: u64,
	/// BotId
	self_id: String,
	/// 用户id
	user_id: String,
	/// 事件联系人
	contact: FriendContact,
	/// 事件发送者
	sender: FriendSender,
	/// 事件内容
	content: PrivateApplyType,
}

impl PrivateApply {
	pub fn new(
		request_builder: RequestBuilder<FriendContact, FriendSender>,
		content: PrivateApplyType,
	) -> Self {
		use std::time::{SystemTime, UNIX_EPOCH};
		let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
		Self {
			event_id: request_builder.event_id,
			time: timestamp,
			self_id: request_builder.self_id,
			user_id: request_builder.user_id,
			contact: request_builder.contact,
			sender: request_builder.sender,
			content,
		}
	}
}

impl EventBase for PrivateApply {
	type ContactType = FriendContact;
	type SenderType = FriendSender;

	fn time(&self) -> u64 {
		self.time
	}

	fn event(&self) -> &str {
		EventType::Request.into()
	}

	fn event_id(&self) -> &str {
		self.event_id.as_str()
	}

	fn sub_event(&self) -> &str {
		RequestSubEvent::PrivateApply.into()
	}

	fn self_id(&self) -> &str {
		self.self_id.as_str()
	}

	fn user_id(&self) -> &str {
		self.user_id.as_str()
	}

	fn contact(&self) -> Self::ContactType {
		self.contact.clone()
	}

	fn sender(&self) -> Self::SenderType {
		self.sender.clone()
	}
}

impl RequestBase for PrivateApply {
	type Content = PrivateApplyType;

	fn notion(&self) -> &str {
		"收到好友申请请求"
	}

	fn content(&self) -> Self::Content {
		self.content.clone()
	}
}

#[cfg(feature = "request")]
#[macro_export]
macro_rules! create_friend_apply {
	(
		$adapter:expr,
		$event_id:expr,
		$self_id:expr,
		$user_id:expr,
		$contact:expr,
		$sender:expr,
		$content:expr,
	) => {{
		let builder = RequestBuilder<FriendContact, FriendSender> {
			event_id: $event_id.into(),
			self_id: $self_id.into(),
			user_id: $user_id.into(),
			contact: $contact,
			sender: $sender,
		};
		let request = $struct_name::new(builder);
		let event = Event::Request(RequestEvent::GroupApply(request));
		send_event(std::sync::Arc::from($adapter), event);
	}};
}

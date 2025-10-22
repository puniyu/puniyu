use crate::request::{RequestBase, RequestSubEvent};
use crate::{EventBase, EventType};
use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

#[derive(Debug, Clone)]
pub struct RequestGroupBuilder {
	pub event_id: String,
	pub self_id: String,
	pub user_id: String,
	pub contact: GroupContact,
	pub sender: GroupSender,
}

#[derive(Debug, Clone)]
pub struct GroupApplyType {
	/// 申请理由
	pub reason: String,
}

#[derive(Debug, Clone)]
pub struct GroupApply {
	/// 事件id
	event_id: String,
	/// 时间戳
	time: u64,
	/// BotId
	self_id: String,
	/// 用户id
	user_id: String,
	/// 事件联系人
	contact: GroupContact,
	/// 事件发送者
	sender: GroupSender,
	/// 事件内容
	content: GroupApplyType,
}

impl GroupApply {
	pub fn new(request_builder: RequestGroupBuilder, content: GroupApplyType) -> Self {
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

	pub fn group_id(&self) -> &str {
		self.contact.peer.as_str()
	}
}

impl EventBase for GroupApply {
	type ContactType = GroupContact;
	type SenderType = GroupSender;

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
		RequestSubEvent::GroupApply.into()
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

impl RequestBase for GroupApply {
	type Content = GroupApplyType;

	fn notion(&self) -> &str {
		"收到入群申请请求"
	}

	fn content(&self) -> Self::Content {
		self.content.clone()
	}
}

#[cfg(feature = "request")]
#[macro_export]
macro_rules! create_group_apply {
	(
		$adapter:expr,
		$event_id:expr,
		$self_id:expr,
		$user_id:expr,
		$contact:expr,
		$sender:expr,
		$content:expr,
	) => {{
		let builder = RequestFriendBuilder {
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

#[derive(Debug, Clone)]
pub struct GroupInviteType {
	pub target_id: String,
}

#[derive(Debug, Clone)]
pub struct GroupInvite {
	/// 事件id
	event_id: String,
	/// 时间戳
	time: u64,
	/// BotId
	self_id: String,
	/// 用户id
	user_id: String,
	/// 事件联系人
	contact: GroupContact,
	/// 事件发送者
	sender: GroupSender,
	/// 事件内容
	content: GroupInviteType,
}

impl GroupInvite {
	pub fn new(request_builder: RequestGroupBuilder, content: GroupInviteType) -> Self {
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

	pub fn group_id(&self) -> &str {
		self.contact.peer.as_str()
	}
}

impl EventBase for GroupInvite {
	type ContactType = GroupContact;
	type SenderType = GroupSender;

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
		RequestSubEvent::GroupApply.into()
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

impl RequestBase for GroupInvite {
	type Content = GroupInviteType;

	fn notion(&self) -> &str {
		"收到入群邀请请求"
	}

	fn content(&self) -> Self::Content {
		self.content.clone()
	}
}

#[cfg(feature = "request")]
#[macro_export]
macro_rules! create_group_invite {
	(
		$adapter:expr,
		$event_id:expr,
		$self_id:expr,
		$user_id:expr,
		$contact:expr,
		$sender:expr,
		$content:expr,
	) => {{
		let builder = RequestFriendBuilder {
			event_id: $event_id.into(),
			self_id: $self_id.into(),
			user_id: $user_id.into(),
			contact: $contact,
			sender: $sender,
		};
		let request = $struct_name::new(builder);
		let event = Event::Request(RequestEvent::GroupInvite(request));
		send_event(std::sync::Arc::from($adapter), event);
	}};
}

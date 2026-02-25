use crate::request::{
	ContentType, GroupApply, GroupInvite, PrivateApply, RequestBase, RequestSubEventType,
};
use crate::{EventBase, EventType};
use puniyu_bot::Bot;
use puniyu_contact::ContactType;
use puniyu_sender::SenderType;

/// 请求事件枚举
///
/// 包含所有类型的请求事件。
#[derive(Debug, Clone)]
pub enum RequestEvent<'r> {
	/// 好友申请
	PrivateApply(PrivateApply<'r>),
	/// 群申请
	GroupApply(GroupApply<'r>),
	/// 邀请入群
	GroupInvite(GroupInvite<'r>),
}

macro_rules! delegate_to_variants {
	($self:ident, $method:ident() -> $ret:ty) => {
		match $self {
			Self::PrivateApply(inner) => inner.$method(),
			Self::GroupApply(inner) => inner.$method(),
			Self::GroupInvite(inner) => inner.$method(),
		}
	};
}

macro_rules! impl_as {
    ($enum_name:ident { $($variant:ident => $method_name:ident),* $(,)? }) => {
        impl $enum_name<'_> {
            $(
                pub fn $method_name(&self) -> Option<&$variant<'_>> {
                    match self {
                        Self::$variant(inner) => Some(inner),
                        _ => None,
                    }
                }
            )*
        }
    };
}

impl_as! {
	RequestEvent {
		PrivateApply => as_private_apply,
		GroupApply => as_group_apply,
		GroupInvite => as_group_invite,
	}
}
impl RequestEvent<'_> {
	/// 获取请求触发时间戳
	///
	/// # 返回值
	///
	/// 返回 Unix 时间戳（秒）
	pub fn time(&self) -> u64 {
		delegate_to_variants!(self, time() -> u64)
	}

	/// 获取事件类型
	///
	/// # 返回值
	///
	/// 返回 `EventType::Request`
	pub fn event(&self) -> &EventType {
		delegate_to_variants!(self, event() -> EventType)
	}

	/// 获取事件 ID
	///
	/// # 返回值
	///
	/// 返回事件的唯一标识符
	pub fn event_id(&self) -> &str {
		delegate_to_variants!(self, event_id() -> &str)
	}

	/// 获取请求子类型
	///
	/// # 返回值
	///
	/// 返回请求的具体子类型
	pub fn sub_event(&self) -> &RequestSubEventType {
		delegate_to_variants!(self, sub_event() -> &RequestSubEvent)
	}

	/// 获取机器人实例
	///
	/// # 返回值
	///
	/// 返回处理该请求的机器人实例引用
	pub fn bot(&self) -> &Bot {
		delegate_to_variants!(self, bot() -> &Bot)
	}

	/// 获取机器人 ID
	///
	/// # 返回值
	///
	/// 返回机器人的唯一标识符
	pub fn self_id(&self) -> &str {
		delegate_to_variants!(self, self_id() -> &str)
	}

	/// 获取用户 ID
	///
	/// # 返回值
	///
	/// 返回发起请求的用户 ID
	pub fn user_id(&self) -> &str {
		delegate_to_variants!(self, user_id() -> &str)
	}

	/// 获取联系人信息
	///
	/// # 返回值
	///
	/// 返回请求相关的联系人信息
	pub fn contact(&self) -> ContactType<'_> {
		match self {
			Self::PrivateApply(inner) => ContactType::from(inner.contact().clone()),
			Self::GroupApply(inner) => ContactType::from(inner.contact().clone()),
			Self::GroupInvite(inner) => ContactType::from(inner.contact().clone()),
		}
	}

	/// 获取发送者信息
	///
	/// # 返回值
	///
	/// 返回发起请求的用户详细信息
	pub fn sender(&self) -> SenderType<'_> {
		match self {
			Self::PrivateApply(inner) => SenderType::from(inner.sender().clone()),
			Self::GroupApply(inner) => SenderType::from(inner.sender().clone()),
			Self::GroupInvite(inner) => SenderType::from(inner.sender().clone()),
		}
	}
}

impl RequestEvent<'_> {
	/// 获取请求消息
	///
	/// # 返回值
	///
	/// 返回请求的文本消息内容
	pub fn request(&self) -> &str {
		delegate_to_variants!(self, request() -> &str)
	}

	/// 获取请求内容
	///
	/// # 返回值
	///
	/// 返回请求的详细内容对象
	pub fn content(&self) -> ContentType {
		match self {
			Self::PrivateApply(inner) => ContentType::from(inner.content().clone()),
			Self::GroupApply(inner) => ContentType::from(inner.content().clone()),
			Self::GroupInvite(inner) => ContentType::from(inner.content().clone()),
		}
	}
}

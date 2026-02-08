mod friend;
#[doc(inline)]
pub use friend::*;
mod group;
#[doc(inline)]
pub use group::*;

use super::EventBase;
use crate::bot::Bot;
use crate::event::EventType;
use strum::{Display, EnumString, IntoStaticStr};
use serde::{Deserialize, Serialize};

#[derive(
	Debug,
	Clone,
	EnumString,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
)]
pub enum RequestSubEvent {
	/// 好友申请
	#[strum(serialize = "privateApply")]
	PrivateApply,
	/// 群申请
	#[strum(serialize = "groupApply")]
	GroupApply,
	/// 邀请入群
	#[strum(serialize = "groupInvite")]
	GroupInvite,
}

#[derive(Debug, Clone)]
pub enum RequestEvent<'r> {
	/// 好友申请
	PrivateApply(PrivateApply<'r>),
	/// 群申请
	GroupApply(GroupApply<'r>),
	/// 邀请入群
	GroupInvite(GroupInvite<'r>),
}

pub trait RequestBase: Send + Sync + EventBase<EventType, RequestSubEvent> {
	type Content;
	/// 请求消息
	fn notion(&self) -> &str;

	/// 请求内容
	fn content(&self) -> &Self::Content;
}

#[derive(Debug, Clone)]
pub struct RequestBuilder<'r, Contact, Sender>
where
	Contact: crate::contact::Contact,
	Sender: crate::sender::Sender,
{
	pub bot: &'r Bot,
	pub event_id: &'r str,
	pub time: u64,
	pub self_id: &'r str,
	pub user_id: &'r str,
	pub contact: &'r Contact,
	pub sender: &'r Sender,
}

mod friend;
pub use friend::*;
mod group;
pub use group::*;

use crate::EventBase;
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, EnumString, Display, IntoStaticStr)]
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
pub enum RequestEvent {
	/// 好友申请
	PrivateApply(PrivateApply),
	/// 群申请
	GroupApply(GroupApply),
	/// 邀请入群
	GroupInvite(GroupInvite),
}

pub trait RequestBase: Send + Sync + EventBase {
	type Content;
	/// 请求消息
	fn notion(&self) -> &str;

	/// 请求内容
	fn content(&self) -> Self::Content;
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SetFriendApplyType {
	/// 同意好友申请
	Agree,
	/// 拒绝好友申请
	Refuse,
}

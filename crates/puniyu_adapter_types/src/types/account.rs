use serde::{Deserialize, Serialize};

/// 好友申请处理动作。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SetFriendApplyType {
	/// 同意好友申请。
	Agree,
	/// 拒绝好友申请。
	Refuse,
}

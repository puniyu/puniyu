use puniyu_sender::Sex;
use serde::{Deserialize, Serialize};

/// 好友资料。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfo {
	/// 用户 ID。
	pub user_id: String,
	/// 用户昵称。
	pub nick: String,
	/// 用户头像地址。
	pub avatar: String,
	/// 用户年龄。
	pub age: Option<u32>,
	/// 用户性别。
	pub sex: Sex,
}

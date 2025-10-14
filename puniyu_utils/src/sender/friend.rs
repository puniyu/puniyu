use super::Sex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FriendSender {
	/// 发送者id
	pub user_id: String,
	/// 用户昵称
	pub nick: String,
	/// 性别
	pub sex: Sex,
	/// 年龄
	pub age: u8,
}

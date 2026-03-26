use puniyu_sender::Sex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfo {
	/// 用户ID
	pub user_id: String,
	/// 用户昵称
	pub nick: String,
	/// 用户头像
	pub avatar: String,
	/// 用户年龄
	pub age: Option<u32>,
	/// 用户性别
	pub sex: Sex,
}

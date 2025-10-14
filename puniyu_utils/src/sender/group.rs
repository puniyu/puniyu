use super::{Role, Sex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupSender {
	/// 发送者id
	pub user_id: String,
	/// 用户昵称
	pub nick: String,
	/// 性别
	pub sex: Sex,
	/// 年龄
	pub age: u8,
	/// 角色
	pub role: Role,
	/// 群名片
	pub card: Option<String>,
	/// 等级
	pub level: Option<String>,
	/// 专属头衔
	pub title: Option<String>,
}

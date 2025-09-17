use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// 性别
pub enum Sex {
	/// 男性
	Male,
	/// 女性
	Female,
	/// 未知
	Unknown,
}
impl Sex {
	pub fn to_sex_str(&self) -> &'static str {
		match self {
			Sex::Male => "male",
			Sex::Female => "female",
			Sex::Unknown => "unknown",
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// 事件发送者角色, 私聊时不存在为None
pub enum Role {
	/// 群主
	Owner,
	/// 管理员
	Admin,
	/// 成员
	Member,
	/// 未知
	Unknown,
}
impl Role {
	pub fn to_role_str(&self) -> &'static str {
		match self {
			Role::Owner => "owner",
			Role::Admin => "admin",
			Role::Member => "member",
			Role::Unknown => "unknown",
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendSender {
	/// 发送者id
	user_id: String,
	/// 用户昵称
	nick: String,
	/// 性别
	sex: Sex,
	/// 年龄
	age: u8,
}

impl FriendSender {
	pub fn new(user_id: String, nick: String, sex: Sex, age: u8) -> Self {
		Self { user_id, nick, sex, age }
	}

	pub fn user_id(&self) -> &str {
		&self.user_id
	}

	pub fn nick(&self) -> &str {
		&self.nick
	}

	pub fn age(&self) -> u8 {
		self.age
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSender {
	/// 发送者id
	user_id: String,
	/// 用户昵称
	nick: String,
	/// 性别
	sex: Sex,
	/// 年龄
	age: u8,
	/// 角色
	role: Role,
	/// 群名片
	card: Option<String>,
	/// 等级
	level: Option<String>,
	/// 专属头衔
	title: Option<String>,
}

/// TODO: 群聊临时会话，频道，频道私信
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sender {
	Friend(FriendSender),
	Group(GroupSender),
}

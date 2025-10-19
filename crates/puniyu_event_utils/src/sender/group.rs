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

#[macro_export]
macro_rules! group_sender {
	(
        user_id: $user_id:expr,
        nick: $nick:expr,
        sex: $sex:expr,
        age: $age:expr,
        role: $role:expr,
        card: $card:expr,
        level: $level:expr,
        title: $title:expr
    ) => {
		GroupSender {
			user_id: $user_id.to_string(),
			nick: $nick.to_string(),
			sex: $sex,
			age: $age,
			role: $role,
			card: Some($card.to_string()),
			level: Some($level.to_string()),
			title: Some($title.to_string()),
		}
	};

	(
        user_id: $user_id:expr,
        nick: $nick:expr,
        sex: $sex:expr,
        age: $age:expr,
        role: $role:expr,
        card: $card:expr,
        level: $level:expr
    ) => {
		GroupSender {
			user_id: $user_id.to_string(),
			nick: $nick.to_string(),
			sex: $sex,
			age: $age,
			role: $role,
			card: Some($card.to_string()),
			level: Some($level.to_string()),
			title: None,
		}
	};

	(
        user_id: $user_id:expr,
        nick: $nick:expr,
        sex: $sex:expr,
        age: $age:expr,
        role: $role:expr,
        card: $card:expr
    ) => {
		GroupSender {
			user_id: $user_id.to_string(),
			nick: $nick.to_string(),
			sex: $sex,
			age: $age,
			role: $role,
			card: Some($card.to_string()),
			level: None,
			title: None,
		}
	};
	(
        user_id: $user_id:expr,
        nick: $nick:expr,
        sex: $sex:expr,
        age: $age:expr,
        role: $role:expr
    ) => {
		GroupSender {
			user_id: $user_id.to_string(),
			nick: $nick.to_string(),
			sex: $sex,
			age: $age,
			role: $role,
			card: None,
			level: None,
			title: None,
		}
	};

	($user_id:expr, $nick:expr, $sex:expr, $age:expr) => {
		GroupSender {
			user_id: $user_id.to_string(),
			nick: $nick.to_string(),
			sex: $sex,
			age: $age,
			role: Role::Unknown,
			card: None,
			level: None,
			title: None,
		}
	};
}

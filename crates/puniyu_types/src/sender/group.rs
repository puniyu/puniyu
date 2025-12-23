use super::{Role, Sender, Sex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupSender {
	/// 发送者id
	pub user_id: String,
	/// 用户昵称
	pub nick: Option<String>,
	/// 性别
	pub sex: Sex,
	/// 年龄
	pub age: u8,
	/// 角色
	pub role: Role,
	/// 群名片
	pub card: Option<String>,
	/// 等级
	pub level: Option<u8>,
	/// 专属头衔
	pub title: Option<String>,
}

impl GroupSender {
	/// 角色
	pub fn role(&self) -> Role {
		self.role.clone()
	}
	/// 群名片
	pub fn card(&self) -> Option<&str> {
		self.card.as_deref()
	}
	/// 等级
	pub fn level(&self) -> Option<u8> {
		self.level
	}
	/// 专属头衔
	pub fn title(&self) -> Option<&str> {
		self.title.as_deref()
	}
}

impl Sender for GroupSender {
	fn user_id(&self) -> &str {
		self.user_id.as_str()
	}
	fn name(&self) -> Option<&str> {
		self.nick.as_deref()
	}
	fn sex(&self) -> Sex {
		self.sex.clone()
	}
	fn age(&self) -> u8 {
		self.age
	}
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
			nick: Some($nick.to_string()),
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
			nick: Some($nick.to_string()),
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
			nick: Some($nick.to_string()),
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
			nick: Some($nick.to_string()),
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
			nick: Some($nick.to_string()),
			sex: $sex,
			age: $age,
			role: Role::Unknown,
			card: None,
			level: None,
			title: None,
		}
	};

	($user_id:expr, $nick:expr, $sex:expr) => {
		GroupSender {
			user_id: $user_id.to_string(),
			nick: Some($nick.to_string()),
			sex: $sex,
			age: 0,
			role: Role::Unknown,
			card: None,
			level: None,
			title: None,
		}
	};

	($user_id:expr, $nick:expr) => {
		GroupSender {
			user_id: $user_id.to_string(),
			nick: Some($nick.to_string()),
			sex: Sex::Unknown,
			age: 0,
			role: Role::Unknown,
			card: None,
			level: None,
			title: None,
		}
	};

	($user_id:expr) => {
		GroupSender {
			user_id: $user_id.to_string(),
			nick: None,
			sex: Sex::Unknown,
			age: 0,
			role: Role::Unknown,
			card: None,
			level: None,
			title: None,
		}
	};
}

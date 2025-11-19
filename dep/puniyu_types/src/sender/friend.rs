use super::{Sender, Sex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FriendSender {
	/// 发送者id
	pub user_id: String,
	/// 用户昵称
	pub nick: Option<String>,
	/// 性别
	pub sex: Sex,
	/// 年龄
	pub age: u8,
}

impl Sender for FriendSender {
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
macro_rules! friend_sender {
	(
        user_id: $user_id:expr,
        nick: $nick:expr,
        sex: $sex:expr,
        age: $age:expr
    ) => {
		FriendSender {
			user_id: $user_id.to_string(),
			nick: Some($nick.to_string()),
			sex: $sex,
			age: $age,
		}
	};

	($user_id:expr, $nick:expr, $sex:expr, $age:expr) => {
		FriendSender {
			user_id: $user_id.to_string(),
			nick: Some($nick.to_string()),
			sex: $sex,
			age: $age,
		}
	};
	($user_id:expr, $nick:expr, $sex:expr) => {
		FriendSender {
			user_id: $user_id.to_string(),
			nick: Some($nick.to_string()),
			sex: $sex,
			age: 0,
		}
	};
	($user_id:expr, $nick:expr) => {
		FriendSender {
			user_id: $user_id.to_string(),
			nick: Some($nick.to_string()),
			sex: Sex::Unknown,
			age: 0,
		}
	};
	($user_id:expr) => {
		FriendSender { user_id: $user_id.to_string(), nick: None, sex: Sex::Unknown, age: 0 }
	};
}

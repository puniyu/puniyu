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
			nick: $nick.to_string(),
			sex: $sex,
			age: $age,
		}
	};

	($user_id:expr, $nick:expr, $sex:expr, $age:expr) => {
		FriendSender {
			user_id: $user_id.to_string(),
			nick: $nick.to_string(),
			sex: $sex,
			age: $age,
		}
	};
}

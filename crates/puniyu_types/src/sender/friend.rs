use super::{Sender, Sex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
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

#[cfg(feature = "sender")]
#[macro_export]
macro_rules! friend_sender {
    ( $( $key:ident : $value:expr ),* $(,)? ) => {{
        FriendSender {
            $(
                $key: friend_sender!(@convert $key, $value),
            )*
            ..FriendSender::default()
        }
    }};

    (@convert user_id, $v:expr) => { $v.to_string() };
    (@convert nick, $v:expr) => { Some($v.to_string()) };
    (@convert sex, $v:expr) => { $v };
    (@convert age, $v:expr) => { $v };
}

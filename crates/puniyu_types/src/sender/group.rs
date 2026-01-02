use super::{Role, Sender, Sex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct GroupSender {
	/// 发送者id
	pub user_id: String,
	/// 用户昵称
	pub nick: Option<String>,
	/// 性别
	pub sex: Sex,
	/// 年龄
	pub age: Option<u8>,
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
	fn age(&self) -> Option<u8> {
		self.age
	}
}

#[cfg(feature = "sender")]
#[macro_export]
macro_rules! group_sender {
    ( $( $key:ident : $value:expr ),* $(,)? ) => {{
        let mut sender = $crate::sender::GroupSender::default();
        $(
            sender.$key = group_sender!(@convert $key, $value);
        )*
        sender
    }};

    (@convert user_id, $v:expr) => { $v.to_string() };
    (@convert nick, None) => { None };
    (@convert nick, $v:expr) => { Some($v.to_string()) };
    (@convert sex, $v:expr) => { $v };
    (@convert age, $v:expr) => { Some($v) };
    (@convert role, $v:expr) => { $v };
    (@convert card, $v:expr) => { Some($v.to_string()) };
    (@convert level, $v:expr) => { Some($v) };
    (@convert title, $v:expr) => { Some($v.to_string()) };
}

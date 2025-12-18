use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// 账户信息
pub struct AccountInfo {
	/// Bot账号的uin
	pub uin: String,
	/// Bot账号的昵称
	pub name: String,
	/// Bot账号的头像URL地址
	pub avatar: String,
}

#[cfg(feature = "account")]
#[macro_export]
macro_rules! account_info {
	(uin: $uin:expr, name: $name:expr,avatar: $avatar:expr) => {
		AccountInfo {
			uin: $uin.to_string(),
			name: $name.to_string(),
			avatar: $avatar.to_string(),
		}
	};
	($uin:expr, $name:expr,$avatar:expr) => {
		AccountInfo {
			uin: $uin.to_string(),
			name: $name.to_string(),
			avatar: $avatar.to_string(),
		}
	};
}

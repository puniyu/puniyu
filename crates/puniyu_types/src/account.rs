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
    ( $( $key:ident : $value:expr ),* $(,)? ) => {{
        let mut info = $crate::account::AccountInfo::default();
        $(
            info.$key = account_info!(@convert $key, $value);
        )*
        info
    }};

    (@convert uin, $v:expr) => { $v.to_string() };
    (@convert name, $v:expr) => { $v.to_string() };
    (@convert avatar, $v:expr) => { $v.to_string() };
}

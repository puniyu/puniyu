use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
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
        let mut info = $crate::account::AccountInfoBuilder::default();
		$(
			builder.$key($value);
		)*
		builder.build().expect("Failed to build AccountInfoBuilder")
    }};

	() => {
		$crate::account::AccountInfoBuilder::default().build().expect("Failed to build AccountInfoBuilder")
	};
}

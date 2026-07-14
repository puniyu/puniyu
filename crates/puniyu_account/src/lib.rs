#![allow(clippy::duplicated_attributes)]
//! # puniyu_account
//!
//! 统一的机器人账户信息类型，描述账号 UIN、昵称与头像数据。
//!
//! ## 特性
//!
//! - 提供 `AccountInfo`
//! - 提供构建宏 `account_info!`
//! - 支持 `serde` 序列化与反序列化
//! - 提供 `AccountInfoBuilder`
//!
//! ## 示例
//!
//! ```rust
//! use bytes::Bytes;
//! use puniyu_account::AccountInfo;
//!
//! let account = AccountInfo {
//!     id: "123456789".into(),
//!     name: "Puniyu".into(),
//!     avatar: Bytes::from_static(b"avatar"),
//! };
//! assert_eq!(account.id, "123456789");
//! ```
//!
//! ```rust
//! use bytes::Bytes;
//! use puniyu_account::account_info;
//!
//! let account = account_info!(
//!     id: "123456789",
//!     name: "Puniyu",
//!     avatar: Bytes::from_static(b"avatar"),
//! );
//! assert_eq!(account.name, "Puniyu");
//! ```

use bon::Builder;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// 机器人账户信息。
///
/// 统一描述当前机器人账号的标识、展示名称和头像数据。
///
/// # 示例
///
/// ```rust
/// use bytes::Bytes;
/// use puniyu_account::AccountInfo;
///
/// let account = AccountInfo {
///     id: "123456789".into(),
///     name: "Puniyu".into(),
///     avatar: Bytes::from_static(b"avatar"),
/// };
///
/// assert_eq!(account.id, "123456789");
/// assert_eq!(account.name, "Puniyu");
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(on(SmolStr, into), on(Bytes, into))]
pub struct AccountInfo {
	/// 机器人账号Id
	pub id: SmolStr,
	/// 机器人账号名称。
	pub name: SmolStr,
	/// 机器人头像数据
	pub avatar: Bytes,
}

/// 构建机器人账户信息。
///
/// 支持命名字段与位置参数两种写法。
///
/// ```rust
/// use bytes::Bytes;
/// use puniyu_account::account_info;
///
/// let named = account_info!(
///     id: "123456789",
///     name: "Puniyu",
///     avatar: Bytes::from_static(b"avatar"),
/// );
/// assert_eq!(named.id, "123456789");
///
/// let positional = account_info!(
///     "123456789",
///     "Puniyu",
///     Bytes::new()
/// );
/// assert_eq!(positional.name, "Puniyu");
/// ```
#[macro_export]
macro_rules! account_info {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::AccountInfo::builder()
            $(
                .$key($value)
            )*
            .build()
    }};
	($uin:expr, $name:expr, $avatar:expr) => {{
		$crate::account_info!(
			id: $uin,
			name: $name,
			avatar: $avatar,
		)
	}};
}

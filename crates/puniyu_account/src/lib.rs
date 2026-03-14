//! # puniyu_account
//!
//! 账户信息定义库，提供机器人账户信息的类型系统。
//!
//! ## 概述
//!
//! `puniyu_account` 提供了机器人账户信息的结构定义，用于存储和管理机器人的基本信息。
//!
//! ## 使用方式
//!
//! ### 创建账户信息
//!
//! ```rust
//! use puniyu_account::AccountInfo;
//!
//! // 手动创建
//! let account = AccountInfo {
//!     uin: "123456789".to_string(),
//!     name: "MyBot".to_string(),
//!     avatar: bytes::Bytes::from("image data"),
//! };
//! ```
//!
//! ### 使用宏创建
//!
//! ```rust
//! use puniyu_account::account_info;
//!
//! let account = account_info!(
//!     uin: "123456789",
//!     name: "MyBot",
//!     avatar: bytes::Bytes::from("image data"),
//! );
//! ```

use bytes::Bytes;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// 账户信息
///
/// 存储机器人账户的基本信息，包括 UIN、昵称和头像。
///
/// # 字段
///
/// - `uin` - Bot 账号的 UIN（用户识别号）
/// - `name` - Bot 账号的昵称
/// - `avatar` - Bot 账号的头像数据（Bytes 类型）
///
/// # 示例
///
/// ```rust
/// use puniyu_account::AccountInfo;
///
/// let account = AccountInfo {
///     uin: "123456789".to_string(),
///     name: "MyBot".to_string(),
///     avatar: bytes::Bytes::from("image data"),
/// };
///
/// assert_eq!(account.uin, "123456789");
/// assert_eq!(account.name, "MyBot");
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct AccountInfo {
	/// Bot 账号的 UIN
	pub uin: String,
	/// Bot 账号的昵称
	pub name: String,
	/// Bot 账号的头像数据
	pub avatar: Bytes,
}

/// 构建账户信息宏
///
/// 提供便捷的方式创建账户信息。支持两种调用方式：命名字段和位置参数。
///
/// # 用法
///
/// ## 命名字段形式
///
/// ```rust
/// use puniyu_account::account_info;
///
/// let account = account_info!(
///     uin: "123456789",
///     name: "MyBot",
///     avatar: bytes::Bytes::from("image data"),
/// );
/// ```
///
/// ## 位置参数形式
///
/// 按照 `uin`、`name`、`avatar` 的顺序传递参数。
///
/// ```rust
/// use puniyu_account::account_info;
///
/// let account = account_info!(
///     "123456789",
///     "MyBot",
///     bytes::Bytes::from("image data")
/// );
/// ```
///
/// ## 空字段处理
///
/// 如果某个字段为空，可以使用空字符串。
///
/// ```rust
/// use puniyu_account::account_info;
///
/// // 命名字段形式
/// let account = account_info!(
///     uin: "123456789",
///     name: "MyBot",
///     avatar: bytes::Bytes::new(),
/// );
///
/// // 位置参数形式
/// let account = account_info!("123456789", "MyBot", bytes::Bytes::new());
/// ```
#[macro_export]
macro_rules! account_info {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        let mut builder = $crate::AccountInfoBuilder::default();
		$(
			builder.$key($value);
		)*
		builder.build().expect("Failed to build AccountInfo")
    }};
	($uin:expr, $name:expr, $avatar:expr) => {{
		$crate::account_info!(
			uin: $uin,
			name: $name,
			avatar: $avatar,
		)
	}};
}

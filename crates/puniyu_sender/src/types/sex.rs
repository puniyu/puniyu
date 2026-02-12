use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 性别枚举
///
/// 表示发送者的性别信息。
///
/// # 变体
///
/// - `Male` - 男性
/// - `Female` - 女性
/// - `Unknown` - 未知性别（默认值）
///
/// # 示例
///
/// ```rust
/// use puniyu_sender::Sex;
/// use std::str::FromStr;
///
/// let sex = Sex::Male;
/// assert!(sex.is_male());
///
/// let sex = Sex::from_str("female").unwrap();
/// assert!(sex.is_female());
/// ```
///
/// # 序列化
///
/// 该枚举实现了 `Serialize` 和 `Deserialize`，可以直接用于 JSON 序列化：
///
/// ```rust
/// use puniyu_sender::Sex;
/// use serde_json;
///
/// let sex = Sex::Female;
/// let json = serde_json::to_string(&sex).unwrap();
/// assert!(json.contains("female"));
/// ```
#[derive(
	Debug,
	Default,
	Clone,
	Copy,
	PartialEq,
	Eq,
	EnumString,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum Sex {
	/// 男性
	#[strum(serialize = "male")]
	Male,
	/// 女性
	#[strum(serialize = "female")]
	Female,
	/// 未知性别
	#[strum(serialize = "unknow")]
	#[default]
	Unknown,
}

impl Sex {
	/// 判断是否为男性
	///
	/// # 返回值
	///
	/// 如果性别为男性则返回 `true`，否则返回 `false`。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::Sex;
	///
	/// let sex = Sex::Male;
	/// assert!(sex.is_male());
	///
	/// let sex = Sex::Female;
	/// assert!(!sex.is_male());
	/// ```
	pub fn is_male(&self) -> bool {
		matches!(self, Sex::Male)
	}

	/// 判断是否为女性
	///
	/// # 返回值
	///
	/// 如果性别为女性则返回 `true`，否则返回 `false`。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::Sex;
	///
	/// let sex = Sex::Female;
	/// assert!(sex.is_female());
	///
	/// let sex = Sex::Male;
	/// assert!(!sex.is_female());
	/// ```
	pub fn is_female(&self) -> bool {
		matches!(self, Sex::Female)
	}

	/// 判断是否为未知性别
	///
	/// # 返回值
	///
	/// 如果性别为未知则返回 `true`，否则返回 `false`。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::Sex;
	///
	/// let sex = Sex::Unknown;
	/// assert!(sex.is_unknown());
	///
	/// let sex = Sex::Male;
	/// assert!(!sex.is_unknown());
	/// ```
	pub fn is_unknown(&self) -> bool {
		matches!(self, Sex::Unknown)
	}
}

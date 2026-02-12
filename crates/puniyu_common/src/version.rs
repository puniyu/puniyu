use serde::{Deserialize, Serialize};
use strum::{Display, IntoStaticStr};

/// 版本信息结构体
///
/// 包含主版本号、次版本号、补丁版本号和版本渠道。
///
/// # 示例
///
/// ```rust
/// use puniyu_common::version::{Version, Channel};
///
/// let version = Version {
///     major: 0,
///     minor: 7,
///     patch: 8,
///     channel: Channel::Stable,
/// };
///
/// println!("{}", version); // 输出: 0.7.8 (Stable)
/// ```
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, derive_more::Display)]
#[display("{major}.{minor}.{patch}:{channel}")]
pub struct Version {
	/// 主版本号
	pub major: u16,
	/// 次版本号
	pub minor: u16,
	/// 补丁版本号
	pub patch: u16,
	/// 版本渠道
	pub channel: Channel,
}

/// 版本渠道枚举
///
/// 定义版本的发布渠道类型。
///
/// # 示例
///
/// ```rust
/// use puniyu_common::version::Channel;
///
/// let stable = Channel::Stable;
/// let nightly = Channel::Nightly;
///
/// assert_eq!(stable.to_string(), "Stable");
/// assert_eq!(nightly.to_string(), "Nightly");
/// ```
#[derive(Debug, Clone,Display, IntoStaticStr, Deserialize, Serialize, PartialEq, Eq)]
pub enum Channel {
	/// 稳定版本
	#[strum(serialize = "Stable")]
	Stable,
	/// 预览版本
	#[strum(serialize = "Nightly")]
	Nightly,
}


/// 当前版本常量
///
/// 从 Cargo 环境变量中自动获取版本信息。
///
/// # 示例
///
/// ```rust
/// use puniyu_common::version::VERSION;
///
/// println!("当前版本: {}", VERSION);
/// ```
pub const VERSION: Version = Version {
	major: parse_u16(env!("CARGO_PKG_VERSION_MAJOR")),
	minor: parse_u16(env!("CARGO_PKG_VERSION_MINOR")),
	patch: parse_u16(env!("CARGO_PKG_VERSION_PATCH")),
	channel: parse_channel(env!("VERSION_CHANNEL")),
};

const fn parse_u16(s: &str) -> u16 {
	let bytes = s.as_bytes();
	let mut result = 0u16;
	let mut i = 0;

	while i < bytes.len() {
		let byte = bytes[i];
		if byte < b'0' || byte > b'9' {
			panic!("Invalid digit in version number");
		}
		let digit = (byte - b'0') as u16;
		result = result * 10 + digit;
		i += 1;
	}

	result
}

const fn parse_channel(s: &str) -> Channel {
	if matches_str_ignore_case(s, "stable") { Channel::Stable } else { Channel::Nightly }
}

const fn matches_str_ignore_case(s: &str, pattern: &str) -> bool {
	let s_bytes = s.as_bytes();
	let p_bytes = pattern.as_bytes();

	if s_bytes.len() != p_bytes.len() {
		return false;
	}

	let mut i = 0;
	while i < s_bytes.len() {
		let s_lower = to_ascii_lowercase(s_bytes[i]);
		let p_lower = to_ascii_lowercase(p_bytes[i]);
		if s_lower != p_lower {
			return false;
		}
		i += 1;
	}

	true
}

const fn to_ascii_lowercase(byte: u8) -> u8 {
	if byte >= b'A' && byte <= b'Z' { byte + 32 } else { byte }
}

//! # puniyu_version
//!
//! 版本号定义库，提供语义化版本号的类型系统。
//!
//! ## 概述
//!
//! `puniyu_version` 提供了符合语义化版本规范的版本号结构定义，用于管理和比较版本信息。
//!
//! ## 使用方式
//!
//! ### 创建版本号
//!
//! ```rust
//! use puniyu_version::Version;
//!
//! // 手动创建
//! let version = Version {
//!     major: 1,
//!     minor: 2,
//!     patch: 3,
//! };
//!
//! // 从字符串创建
//! let version: Version = "1.2.3".into();
//!
//! // 使用默认版本（0.0.1）
//! let version = Version::default();
//! ```
//!
//! ### 版本号显示
//!
//! ```rust
//! use puniyu_version::Version;
//!
//! let version = Version {
//!     major: 1,
//!     minor: 2,
//!     patch: 3,
//! };
//!
//! println!("Version: {}", version); // 输出: Version: 1.2.3
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// 版本号
///
/// 符合语义化版本规范的版本号结构，包含主版本号、次版本号和补丁版本号。
///
/// # 字段
///
/// - `major` - 主版本号，当做了不兼容的 API 修改时递增
/// - `minor` - 次版本号，当做了向下兼容的功能性新增时递增
/// - `patch` - 补丁版本号，当做了向下兼容的问题修正时递增
///
/// # 示例
///
/// ```rust
/// use puniyu_version::Version;
///
/// let version = Version {
///     major: 1,
///     minor: 2,
///     patch: 3,
/// };
///
/// assert_eq!(version.to_string(), "1.2.3");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{major}.{minor}.{patch}")]
pub struct Version {
	/// 主版本号
	pub major: u16,
	/// 次版本号
	pub minor: u16,
	/// 补丁版本号
	pub patch: u16,
}

impl Version {
	pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
		Self { major, minor, patch }
	}
}

impl Default for Version {
	/// 创建默认版本号（0.0.1）
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_version::Version;
	///
	/// let version = Version::default();
	/// assert_eq!(version.major, 0);
	/// assert_eq!(version.minor, 0);
	/// assert_eq!(version.patch, 1);
	/// ```
	fn default() -> Self {
		Self { major: 0, minor: 0, patch: 1 }
	}
}

impl FromStr for Version {
	type Err = std::convert::Infallible;

	/// 从字符串解析版本号
	///
	/// 字符串格式应为 `major.minor.patch`。如果解析失败，返回默认版本号（0.0.1）。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_version::Version;
	/// use std::str::FromStr;
	///
	/// let version = Version::from_str("1.2.3").unwrap();
	/// assert_eq!(version.major, 1);
	/// assert_eq!(version.minor, 2);
	/// assert_eq!(version.patch, 3);
	///
	/// // 解析失败返回默认版本
	/// let version = Version::from_str("invalid").unwrap();
	/// assert_eq!(version, Version::default());
	/// ```
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts = s.trim().split('.').collect::<Vec<_>>();
		if parts.len() != 3 {
			return Ok(Version::default());
		}

		Ok(Version {
			major: parts[0].parse().unwrap_or_default(),
			minor: parts[1].parse().unwrap_or_default(),
			patch: parts[2].parse().unwrap_or_default(),
		})
	}
}

impl AsRef<Self> for Version {
	fn as_ref(&self) -> &Self {
		self
	}
}

impl From<&'static str> for Version {
	/// 从字符串切片创建版本号
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_version::Version;
	///
	/// let version: Version = "1.2.3".into();
	/// assert_eq!(version.major, 1);
	/// assert_eq!(version.minor, 2);
	/// assert_eq!(version.patch, 3);
	/// ```
	fn from(s: &'static str) -> Self {
		Self::from_str(s).unwrap_or_default()
	}
}

impl From<String> for Version {
	/// 从 String 创建版本号
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_version::Version;
	///
	/// let s = String::from("1.2.3");
	/// let version: Version = s.into();
	/// assert_eq!(version.major, 1);
	/// ```
	fn from(s: String) -> Self {
		Self::from_str(&s).unwrap_or_default()
	}
}

impl From<Version> for String {
	/// 将版本号转换为 String
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_version::Version;
	///
	/// let version = Version {
	///     major: 1,
	///     minor: 2,
	///     patch: 3,
	/// };
	/// let s: String = version.into();
	/// assert_eq!(s, "1.2.3");
	/// ```
	fn from(v: Version) -> Self {
		v.to_string()
	}
}

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
	/// 主版本号
	pub major: u16,
	/// 次版本号
	pub minor: u16,
	/// 补丁版本号
	pub patch: u16,
}

impl Default for Version {
	fn default() -> Self {
		Self { major: 0, minor: 0, patch: 1 }
	}
}

impl fmt::Display for Version {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
	}
}

impl AsRef<Self> for Version {
	fn as_ref(&self) -> &Self {
		self
	}
}
impl From<&'static str> for Version {
	fn from(s: &'static str) -> Self {
		let mut v = s.split('.');
		Self {
			major: v.next().unwrap_or_default().parse().unwrap_or_default(),
			minor: v.next().unwrap_or_default().parse().unwrap_or_default(),
			patch: v.next().unwrap_or_default().parse().unwrap_or_default(),
		}
	}
}

impl From<String> for Version {
	fn from(s: String) -> Self {
		let mut parts = s.split('.');
		Self {
			major: parts.next().unwrap_or_default().parse().unwrap_or_default(),
			minor: parts.next().unwrap_or_default().parse().unwrap_or_default(),
			patch: parts.next().unwrap_or_default().parse().unwrap_or_default(),
		}
	}
}

impl From<Version> for String {
	fn from(v: Version) -> Self {
		format!("{}.{}.{}", v.major, v.minor, v.patch)
	}
}

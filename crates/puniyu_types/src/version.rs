use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
	/// 主版本号
	pub major: &'static str,
	/// 次版本号
	pub minor: &'static str,
	/// 补丁版本号
	pub patch: &'static str,
}

impl Default for Version {
	fn default() -> Self {
		Version::from("0.0.1")
	}
}

impl fmt::Display for Version {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
	}
}

impl From<&'static str> for Version {
	fn from(s: &'static str) -> Self {
		let mut v = s.split('.');
		Version { major: v.next().unwrap(), minor: v.next().unwrap(), patch: v.next().unwrap() }
	}
}

impl From<String> for Version {
	fn from(s: String) -> Self {
		let leaked = Box::leak(s.into_boxed_str());
		let mut v = leaked.split('.');
		Version { major: v.next().unwrap(), minor: v.next().unwrap(), patch: v.next().unwrap() }
	}
}

impl From<Version> for String {
	fn from(v: Version) -> Self {
		format!("{}.{}.{}", v.major, v.minor, v.patch)
	}
}

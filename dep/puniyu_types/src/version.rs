use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
	/// 主版本号
	pub major: String,
	/// 次版本号
	pub minor: String,
	/// 补丁版本号
	pub patch: String,
}

impl fmt::Display for Version {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
	}
}

impl From<&'static str> for Version {
	fn from(s: &'static str) -> Self {
		let mut v = s.split('.');
		Version {
			major: v.next().unwrap().to_string(),
			minor: v.next().unwrap().to_string(),
			patch: v.next().unwrap().to_string(),
		}
	}
}

impl From<String> for Version {
	fn from(s: String) -> Self {
		let mut v = s.split('.');
		Version {
			major: v.next().unwrap().to_string(),
			minor: v.next().unwrap().to_string(),
			patch: v.next().unwrap().to_string(),
		}
	}
}

impl From<Version> for String {
    fn from(v: Version) -> Self {
        format!("{}.{}.{}", v.major, v.minor, v.patch)
    }
}

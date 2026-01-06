use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

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

impl FromStr for Version {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts= s.split('.').collect::<Vec<_>>();
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
	fn from(s: &'static str) -> Self {
		Self::from_str(s).unwrap_or_default()
	}
}

impl From<String> for Version {
	fn from(s: String) -> Self {
		Self::from_str(&s).unwrap_or_default()
	}
}

impl From<Version> for String {
	fn from(v: Version) -> Self {
		v.to_string()
	}
}

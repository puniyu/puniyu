use std::str::FromStr;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{major}.{minor}.{patch}")]
pub struct Version {
	/// 主版本号
	pub major: u64,
	/// 次版本号
	pub minor: u64,
	/// 补丁版本号
	pub patch: u64,
}

impl Version {
	pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
		Self { major, minor, patch }
	}
}

impl From<semver::Version> for Version {
	fn from(v: semver::Version) -> Self {
		Self { major: v.major, minor: v.minor, patch: v.patch }
	}
}

impl From<Version> for semver::Version {
	fn from(v: Version) -> Self {
		semver::Version::new(v.major, v.minor, v.patch)
	}
}

impl FromStr for Version {
	type Err = semver::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		semver::Version::from_str(s).map(Into::into)
	}
}

use std::fmt;

#[derive(Debug)]
pub struct Version {
	/// 主版本号
	pub major: &'static str,
	/// 次版本号
	pub minor: &'static str,
	/// 补丁版本号
	pub patch: &'static str,
	/// 版本渠道
	/// `Preview` 预览版本
	/// `Stable` 稳定版本
	pub channel: &'static str,
}

impl Version {
	pub const fn new(
		major: &'static str,
		minor: &'static str,
		patch: &'static str,
		channel: &'static str,
	) -> Self {
		Self { major, minor, patch, channel }
	}
}

impl fmt::Display for Version {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
	}
}

pub const VERSION: Version = Version::new(
	env!("CORE_VERSION_MAJOR"),
	env!("CORE_VERSION_MINOR"),
	env!("CORE_VERSION_PATCH"),
	env!("CORE_VERSION_CHANNEL"),
);

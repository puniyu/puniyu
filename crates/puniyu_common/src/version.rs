use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{Display, IntoStaticStr};

const fn str_to_u16(s: &str) -> u16 {
	let bytes = s.as_bytes();
	let mut result = 0u16;
	let mut i = 0;

	while i < bytes.len() {
		let digit = bytes[i] - b'0';
		if digit > 9 {
			panic!("Invalid digit");
		}
		if result > (u16::MAX - digit as u16) / 10 {
			panic!("Overflow");
		}
		result = result * 10 + digit as u16;
		i += 1;
	}

	result
}

const fn is_stable(s: &str) -> bool {
	let bytes = s.as_bytes();
	if bytes.len() != 6 {
		return false;
	}

	let lower_bytes = [
		if bytes[0] >= b'A' && bytes[0] <= b'Z' { bytes[0] + 32 } else { bytes[0] },
		if bytes[1] >= b'A' && bytes[1] <= b'Z' { bytes[1] + 32 } else { bytes[1] },
		if bytes[2] >= b'A' && bytes[2] <= b'Z' { bytes[2] + 32 } else { bytes[2] },
		if bytes[3] >= b'A' && bytes[3] <= b'Z' { bytes[3] + 32 } else { bytes[3] },
		if bytes[4] >= b'A' && bytes[4] <= b'Z' { bytes[4] + 32 } else { bytes[4] },
		if bytes[5] >= b'A' && bytes[5] <= b'Z' { bytes[5] + 32 } else { bytes[5] },
	];

	lower_bytes[0] == b's'
		&& lower_bytes[1] == b't'
		&& lower_bytes[2] == b'a'
		&& lower_bytes[3] == b'b'
		&& lower_bytes[4] == b'l'
		&& lower_bytes[5] == b'e'
}

const fn str_to_channel(s: &str) -> Channel {
	if is_stable(s) { Channel::Stable } else { Channel::Nightly }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Version {
	/// 主版本号
	pub major: u16,
	/// 次版本号
	pub minor: u16,
	/// 补丁版本号
	pub patch: u16,
	/// 版本渠道
	/// `Nightly` 预览版本
	/// `Stable` 稳定版本
	pub channel: Channel,
}

#[derive(Debug, Clone, Display, IntoStaticStr, Deserialize, Serialize)]
pub enum Channel {
	#[strum(serialize = "Stable")]
	Stable,
	#[strum(serialize = "Nightly")]
	Nightly,
}

impl fmt::Display for Version {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}.{}.{} ({})", self.major, self.minor, self.patch, self.channel)
	}
}

pub const VERSION: Version = Version {
	major: str_to_u16(env!("CARGO_PKG_VERSION_MAJOR")),
	minor: str_to_u16(env!("CARGO_PKG_VERSION_MINOR")),
	patch: str_to_u16(env!("CARGO_PKG_VERSION_PATCH")),
	channel: str_to_channel(env!("VERSION_CHANNEL")),
};

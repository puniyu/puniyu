use puniyu_types::version::Version;

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

pub const VERSION: Version = Version {
	major: str_to_u16(env!("CARGO_PKG_VERSION_MAJOR")),
	minor: str_to_u16(env!("CARGO_PKG_VERSION_MINOR")),
	patch: str_to_u16(env!("CARGO_PKG_VERSION_PATCH")),
};

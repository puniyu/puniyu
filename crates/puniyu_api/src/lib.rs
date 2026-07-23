pub mod contact;
pub mod element;
pub mod error;
pub mod event;
pub mod handler;
pub mod message;
pub mod sender;
pub mod session;

#[macro_export]
macro_rules! pkg_name {
	() => {{ env!("CARGO_PKG_NAME") }};
}

#[macro_export]
macro_rules! pkg_version {
	() => {{
		const fn parse(s: &str) -> u64 {
			const fn inner(bytes: &[u8], i: usize, r: u64) -> u64 {
				if i >= bytes.len() {
					r
				} else {
					inner(bytes, i + 1, r * 10 + (bytes[i] - b'0') as u64)
				}
			}
			inner(s.as_bytes(), 0, 0)
		}
		::semver::Version::new(
			parse(env!("CARGO_PKG_VERSION_MAJOR")),
			parse(env!("CARGO_PKG_VERSION_MINOR")),
			parse(env!("CARGO_PKG_VERSION_PATCH")),
		)
	}};
}

#[macro_export]
macro_rules! pkg_description {
	() => {{ env!("CARGO_PKG_DESCRIPTION") }};
}

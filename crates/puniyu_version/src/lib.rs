use semver::Version;

const MAJOR: u64 = const_str::parse!(env!("CARGO_PKG_VERSION_MAJOR"), u64);
const MINOR: u64 = const_str::parse!(env!("CARGO_PKG_VERSION_MINOR"), u64);
const PATCH: u64 = const_str::parse!(env!("CARGO_PKG_VERSION_PATCH"), u64);

pub const VERSION: Version = Version::new(MAJOR, MINOR, PATCH);

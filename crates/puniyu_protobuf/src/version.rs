include!(concat!(env!("OUT_DIR"), "/puniyu.version.rs"));

impl From<Version> for puniyu_version::Version {
	fn from(value: Version) -> Self {
		Self { major: value.major, minor: value.minor, patch: value.patch }
	}
}

impl From<puniyu_version::Version> for Version {
	fn from(value: puniyu_version::Version) -> Self {
		Self { major: value.major, minor: value.minor, patch: value.patch }
	}
}

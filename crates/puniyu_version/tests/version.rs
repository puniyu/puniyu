use puniyu_version::Version;
use std::str::FromStr;

#[test]
fn test_version_creation() {
	let version = Version { major: 1, minor: 2, patch: 3 };

	assert_eq!(version.major, 1);
	assert_eq!(version.minor, 2);
	assert_eq!(version.patch, 3);
}

#[test]
fn test_version_default() {
	let version = Version::default();

	assert_eq!(version.major, 0);
	assert_eq!(version.minor, 0);
	assert_eq!(version.patch, 1);
}

#[test]
fn test_version_display() {
	let version = Version { major: 1, minor: 2, patch: 3 };

	assert_eq!(version.to_string(), "1.2.3");
	assert_eq!(format!("{}", version), "1.2.3");
}

#[test]
fn test_version_from_str() {
	let version = Version::from_str("1.2.3").unwrap();

	assert_eq!(version.major, 1);
	assert_eq!(version.minor, 2);
	assert_eq!(version.patch, 3);
}

#[test]
fn test_version_from_str_invalid() {
	let version = Version::from_str("invalid").unwrap();
	assert_eq!(version, Version::default());

	let version = Version::from_str("1.2").unwrap();
	assert_eq!(version, Version::default());

	let version = Version::from_str("1.2.3.4").unwrap();
	assert_eq!(version, Version::default());
}

#[test]
fn test_version_from_str_partial_invalid() {
	let version = Version::from_str("1.a.3").unwrap();
	assert_eq!(version.major, 1);
	assert_eq!(version.minor, 0);
	assert_eq!(version.patch, 3);
}

#[test]
fn test_version_from_static_str() {
	let version: Version = "1.2.3".into();

	assert_eq!(version.major, 1);
	assert_eq!(version.minor, 2);
	assert_eq!(version.patch, 3);
}

#[test]
fn test_version_from_string() {
	let s = String::from("1.2.3");
	let version: Version = s.into();

	assert_eq!(version.major, 1);
	assert_eq!(version.minor, 2);
	assert_eq!(version.patch, 3);
}

#[test]
fn test_version_to_string() {
	let version = Version { major: 1, minor: 2, patch: 3 };
	let s: String = version.into();

	assert_eq!(s, "1.2.3");
}

#[test]
fn test_version_clone() {
	let version1 = Version { major: 1, minor: 2, patch: 3 };
	let version2 = version1.clone();

	assert_eq!(version1, version2);
}

#[test]
fn test_version_equality() {
	let version1 = Version { major: 1, minor: 2, patch: 3 };
	let version2 = Version { major: 1, minor: 2, patch: 3 };
	let version3 = Version { major: 2, minor: 0, patch: 0 };

	assert_eq!(version1, version2);
	assert_ne!(version1, version3);
}

#[test]
fn test_version_debug() {
	let version = Version { major: 1, minor: 2, patch: 3 };
	let debug_str = format!("{:?}", version);

	assert!(debug_str.contains("Version"));
	assert!(debug_str.contains("major"));
	assert!(debug_str.contains("1"));
}

#[test]
fn test_version_as_ref() {
	let version = Version { major: 1, minor: 2, patch: 3 };
	let version_ref: &Version = version.as_ref();

	assert_eq!(version_ref.major, 1);
}

#[test]
fn test_version_zero() {
	let version = Version { major: 0, minor: 0, patch: 0 };

	assert_eq!(version.to_string(), "0.0.0");
}

#[test]
fn test_version_large_numbers() {
	let version = Version { major: 999, minor: 999, patch: 999 };

	assert_eq!(version.to_string(), "999.999.999");
}

#[test]
fn test_version_serialization() {
	let version = Version { major: 1, minor: 2, patch: 3 };

	let json = serde_json::to_string(&version).unwrap();
	let deserialized: Version = serde_json::from_str(&json).unwrap();

	assert_eq!(version, deserialized);
}

#[test]
fn test_version_major_only() {
	let version = Version { major: 5, minor: 0, patch: 0 };

	assert_eq!(version.to_string(), "5.0.0");
}

#[test]
fn test_version_minor_only() {
	let version = Version { major: 0, minor: 5, patch: 0 };

	assert_eq!(version.to_string(), "0.5.0");
}

#[test]
fn test_version_patch_only() {
	let version = Version { major: 0, minor: 0, patch: 5 };

	assert_eq!(version.to_string(), "0.0.5");
}

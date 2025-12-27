use puniyu_types::version::Version;

#[test]
fn test_version_from_static_str() {
	let version: Version = "1.2.3".into();
	assert_eq!(version.major, 1);
	assert_eq!(version.minor, 2);
	assert_eq!(version.patch, 3);
}

#[test]
fn test_version_from_string() {
	let version_string = String::from("10.20.30");
	let version: Version = version_string.into();
	assert_eq!(version.major, 10);
	assert_eq!(version.minor, 20);
	assert_eq!(version.patch, 30);
}

#[test]
fn test_version_with_large_numbers() {
	let version: Version = "100.200.300".into();
	assert_eq!(version.major, 100);
	assert_eq!(version.minor, 200);
	assert_eq!(version.patch, 300);
}

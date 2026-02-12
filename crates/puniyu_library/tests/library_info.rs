use puniyu_library::{Error, LibraryInfo};
use std::path::PathBuf;

#[test]
fn test_library_info_invalid_path() {
	let path = PathBuf::from("");
	let result = LibraryInfo::new(path);
	assert!(result.is_err());
}

#[test]
fn test_library_info_nonexistent_file() {
	let path = PathBuf::from("nonexistent_library.dll");
	let result = LibraryInfo::new(path);
	assert!(result.is_err());

	if let Err(Error::LoadFailed { path, .. }) = result {
		assert!(path.to_string_lossy().contains("nonexistent"));
	}
}

#[test]
fn test_error_display() {
	let err = Error::NotFound("test_lib".to_string());
	assert_eq!(err.to_string(), "library not found: test_lib");

	let err = Error::Exists("test_lib".to_string());
	assert_eq!(err.to_string(), "library already exists: test_lib");

	let err = Error::InUse("test_lib".to_string());
	assert_eq!(err.to_string(), "library in use: test_lib");
}

#[test]
fn test_error_debug() {
	let err = Error::NotFound("test".to_string());
	let debug_str = format!("{:?}", err);
	assert!(debug_str.contains("NotFound"));
}

#[test]
fn test_invalid_path_error() {
	let path = PathBuf::from("invalid/path");
	let err = Error::InvalidPath(path.clone());
	assert!(err.to_string().contains("invalid library path"));
	assert!(err.to_string().contains("invalid"));
}

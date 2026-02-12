use puniyu_library::Error;
use std::path::PathBuf;

#[test]
fn test_not_found_error() {
	let error = Error::NotFound("test_lib".to_string());
	assert_eq!(error.to_string(), "library not found: test_lib");
}

#[test]
fn test_exists_error() {
	let error = Error::Exists("test_lib".to_string());
	assert_eq!(error.to_string(), "library already exists: test_lib");
}

#[test]
fn test_in_use_error() {
	let error = Error::InUse("test_lib".to_string());
	assert_eq!(error.to_string(), "library in use: test_lib");
}

#[test]
fn test_invalid_path_error() {
	let path = PathBuf::from("/invalid/path");
	let error = Error::InvalidPath(path.clone());
	let error_str = error.to_string();

	assert!(error_str.contains("invalid library path"));
	assert!(error_str.contains("invalid"));
}

#[test]
fn test_error_with_empty_string() {
	let error = Error::NotFound(String::new());
	assert_eq!(error.to_string(), "library not found: ");
}

#[test]
fn test_error_with_unicode() {
	let error = Error::NotFound("测试库".to_string());
	assert_eq!(error.to_string(), "library not found: 测试库");
}

#[test]
fn test_error_with_special_chars() {
	let error = Error::Exists("lib@123.dll".to_string());
	assert_eq!(error.to_string(), "library already exists: lib@123.dll");
}

#[test]
fn test_error_debug_format() {
	let error = Error::NotFound("test".to_string());
	let debug_str = format!("{:?}", error);

	assert!(debug_str.contains("NotFound"));
	assert!(debug_str.contains("test"));
}

#[test]
fn test_error_match() {
	let error = Error::InUse("lib".to_string());

	match error {
		Error::InUse(name) => assert_eq!(name, "lib"),
		_ => panic!("Expected InUse error"),
	}
}

#[test]
fn test_error_propagation() {
	fn inner() -> Result<(), Error> {
		Err(Error::NotFound("lib".to_string()))
	}

	fn outer() -> Result<(), Error> {
		inner()?;
		Ok(())
	}

	let result = outer();
	assert!(result.is_err());
}

#[test]
fn test_invalid_path_with_empty() {
	let path = PathBuf::new();
	let error = Error::InvalidPath(path);
	assert!(error.to_string().contains("invalid library path"));
}

#[test]
fn test_invalid_path_with_relative() {
	let path = PathBuf::from("./relative/path.dll");
	let error = Error::InvalidPath(path);
	let error_str = error.to_string();

	assert!(error_str.contains("invalid library path"));
	assert!(error_str.contains("relative"));
}

#[test]
fn test_error_clone_string() {
	let error1 = Error::NotFound("test".to_string());
	let error2 = Error::NotFound("test".to_string());

	assert_eq!(error1.to_string(), error2.to_string());
}

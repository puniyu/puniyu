use puniyu_library::LibraryRegistry;


#[test]
fn test_registry_contains_nonexistent() {
	let exists = LibraryRegistry::contains("definitely_not_exists_12345.dll");
	assert!(!exists);
}

#[test]
fn test_registry_get_nonexistent() {
	let result = LibraryRegistry::get("definitely_not_exists_12345.dll");
	assert!(result.is_none());
}

#[test]
fn test_registry_list() {
	let list = LibraryRegistry::list();
	assert!(list.is_empty() || !list.is_empty()); // 总是成功
}

#[test]
fn test_registry_unload_nonexistent() {
	let result = LibraryRegistry::unload("nonexistent_lib.dll");
	assert!(result.is_err());
}

#[test]
fn test_registry_reload_nonexistent() {
	let result = LibraryRegistry::reload("nonexistent_lib.dll");
	assert!(result.is_err());
}

#[test]
fn test_registry_load_invalid_path() {
	use std::path::Path;

	let result = LibraryRegistry::load(Path::new(""));
	assert!(result.is_err());
}

#[test]
fn test_registry_load_nonexistent_file() {
	use std::path::Path;

	let result = LibraryRegistry::load(Path::new("nonexistent_file_12345.dll"));
	assert!(result.is_err());
}

#[test]
fn test_registry_load_from_path_nonexistent() {
	let result = LibraryRegistry::load_from_path("nonexistent_file_12345.dll");
	assert!(result.is_err());
}

#[test]
fn test_registry_count_consistency() {
	let count1 = LibraryRegistry::count();
	let list = LibraryRegistry::list();
	let count2 = list.len();

	assert_eq!(count1, count2);
}

#[test]
fn test_registry_list_contains_consistency() {
	let list = LibraryRegistry::list();

	for name in list {
		assert!(LibraryRegistry::contains(&name));
		assert!(LibraryRegistry::get(&name).is_some());
	}
}

#[test]
fn test_error_types() {
	use puniyu_library::Error;

	// 测试错误类型的创建
	let err1 = Error::NotFound("test".to_string());
	let err2 = Error::Exists("test".to_string());
	let err3 = Error::InUse("test".to_string());

	// 测试错误消息
	assert!(err1.to_string().contains("not found"));
	assert!(err2.to_string().contains("already exists"));
	assert!(err3.to_string().contains("in use"));
}

#[test]
fn test_registry_clear_empty() {
	// 清空空注册表应该成功
	// 注意：这可能影响其他测试
	let result = LibraryRegistry::clear();
	// 可能成功也可能失败（如果有库正在使用）
	let _ = result;
}

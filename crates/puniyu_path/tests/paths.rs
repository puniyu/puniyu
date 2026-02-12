use puniyu_path::*;
use std::path::PathBuf;

// 初始化测试环境
fn init_test_env() {
	use puniyu_common::APP_NAME;

	// 初始化工作目录
	let _ = WORKING_DIR.set(PathBuf::from("/test/work"));

	// 初始化应用名称
	let _ = APP_NAME.set("testapp".to_string());
}

#[test]
fn test_working_dir() {
	init_test_env();

	let working_dir = WORKING_DIR.get().unwrap();
	assert_eq!(working_dir, &PathBuf::from("/test/work"));
}

#[test]
fn test_base_dir() {
	init_test_env();

	let base_dir = BASE_DIR.as_path();
	assert_eq!(base_dir, PathBuf::from("/test/work"));
}

#[test]
fn test_app_dir() {
	init_test_env();

	let app_dir = APP_DIR.as_path();
	let expected = PathBuf::from("/test/work").join("@testapp");
	assert_eq!(app_dir, expected);
}

#[test]
fn test_log_dir() {
	init_test_env();

	let log_dir = LOG_DIR.as_path();
	let expected = PathBuf::from("/test/work").join("@testapp").join("logs");
	assert_eq!(log_dir, expected);
}

#[test]
fn test_config_dir() {
	init_test_env();

	let config_dir = CONFIG_DIR.as_path();
	let expected = PathBuf::from("/test/work").join("@testapp").join("config");
	assert_eq!(config_dir, expected);
}

#[test]
fn test_temp_dir() {
	init_test_env();

	let temp_dir = TEMP_DIR.as_path();
	let expected = PathBuf::from("/test/work").join("@testapp").join("temp");
	assert_eq!(temp_dir, expected);
}

#[test]
fn test_plugin_dir() {
	init_test_env();

	let plugin_dir = PLUGIN_DIR.as_path();
	let expected = PathBuf::from("/test/work").join("plugins");
	assert_eq!(plugin_dir, expected);
}

#[test]
fn test_adapter_dir() {
	init_test_env();

	let adapter_dir = ADAPTER_DIR.as_path();
	let expected = PathBuf::from("/test/work").join("adapters");
	assert_eq!(adapter_dir, expected);
}

#[test]
fn test_data_dir() {
	init_test_env();

	let data_dir = DATA_DIR.as_path();
	let expected = PathBuf::from("/test/work").join("@testapp").join("data");
	assert_eq!(data_dir, expected);
}

#[test]
fn test_resource_dir() {
	init_test_env();

	let resource_dir = RESOURCE_DIR.as_path();
	let expected = PathBuf::from("/test/work").join("@testapp").join("resources");
	assert_eq!(resource_dir, expected);
}

#[test]
fn test_plugin_config_dir() {
	init_test_env();

	let plugin_config_dir = plugin::CONFIG_DIR.as_path();
	let expected = PathBuf::from("/test/work").join("@testapp").join("config").join("plugins");
	assert_eq!(plugin_config_dir, expected);
}

#[test]
fn test_plugin_data_dir() {
	init_test_env();

	let plugin_data_dir = plugin::DATA_DIR.as_path();
	let expected = PathBuf::from("/test/work").join("@testapp").join("data").join("plugins");
	assert_eq!(plugin_data_dir, expected);
}

#[test]
fn test_plugin_resource_dir() {
	init_test_env();

	let plugin_resource_dir = plugin::RESOURCE_DIR.as_path();
	let expected = PathBuf::from("/test/work").join("@testapp").join("resources").join("plugins");
	assert_eq!(plugin_resource_dir, expected);
}

#[test]
fn test_plugin_temp_dir() {
	init_test_env();

	let plugin_temp_dir = plugin::TEMP_DIR.as_path();
	let expected = PathBuf::from("/test/work").join("@testapp").join("temp").join("plugins");
	assert_eq!(plugin_temp_dir, expected);
}

#[test]
fn test_adapter_config_dir() {
	init_test_env();

	let adapter_config_dir = adapter::CONFIG_DIR.as_path();
	let expected = PathBuf::from("/test/work").join("@testapp").join("config").join("adapters");
	assert_eq!(adapter_config_dir, expected);
}

#[test]
fn test_adapter_data_dir() {
	init_test_env();

	let adapter_data_dir = adapter::DATA_DIR.as_path();
	let expected = PathBuf::from("/test/work").join("@testapp").join("data").join("adapters");
	assert_eq!(adapter_data_dir, expected);
}

#[test]
fn test_adapter_temp_dir() {
	init_test_env();

	let adapter_temp_dir = adapter::TEMP_DIR.as_path();
	let expected = PathBuf::from("/test/work").join("@testapp").join("temp").join("adapters");
	assert_eq!(adapter_temp_dir, expected);
}

#[test]
fn test_path_exists() {
	init_test_env();

	let _ = BASE_DIR.as_path();
	let _ = APP_DIR.as_path();
	let _ = LOG_DIR.as_path();
	let _ = CONFIG_DIR.as_path();
	let _ = TEMP_DIR.as_path();
	let _ = PLUGIN_DIR.as_path();
	let _ = ADAPTER_DIR.as_path();
	let _ = DATA_DIR.as_path();
	let _ = RESOURCE_DIR.as_path();
}

#[test]
fn test_path_components() {
	init_test_env();

	let app_dir = APP_DIR.as_path();
	assert!(app_dir.ends_with("@testapp"));

	let log_dir = LOG_DIR.as_path();
	assert!(log_dir.ends_with("logs"));

	let config_dir = CONFIG_DIR.as_path();
	assert!(config_dir.ends_with("config"));

	let plugin_dir = PLUGIN_DIR.as_path();
	assert!(plugin_dir.ends_with("plugins"));

	let adapter_dir = ADAPTER_DIR.as_path();
	assert!(adapter_dir.ends_with("adapters"));
}

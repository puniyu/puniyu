use puniyu_common::path::*;

#[test]
fn test_base_dir() {
	assert!(!BASE_DIR.as_os_str().is_empty(), "BASE_DIR should not be empty");
}

#[test]
fn test_app_dir() {
	assert!(!APP_DIR.as_os_str().is_empty(), "APP_DIR should not be empty");
}

#[test]
fn test_log_dir() {
	assert!(!LOG_DIR.as_os_str().is_empty(), "LOG_DIR should not be empty");
}

#[test]
fn test_config_dir() {
	assert!(!CONFIG_DIR.as_os_str().is_empty(), "CONFIG_DIR should not be empty");
}

#[test]
fn test_temp_dir() {
	assert!(!TEMP_DIR.as_os_str().is_empty(), "TEMP_DIR should not be empty");
}

#[test]
fn test_plugin_dir() {
	assert!(!PLUGIN_DIR.as_os_str().is_empty(), "PLUGIN_DIR should not be empty");
}

#[test]
fn test_data_dir() {
	assert!(!DATA_DIR.as_os_str().is_empty(), "DATA_DIR should not be empty");
}

#[test]
fn test_plugin_data_dir() {
	assert!(!PLUGIN_DATA_DIR.as_os_str().is_empty(), "PLUGIN_DATA_DIR should not be empty");
}

#[test]
fn test_adapter_data_dir() {
	assert!(!ADAPTER_DATA_DIR.as_os_str().is_empty(), "ADAPTER_DATA_DIR should not be empty");
}

//! 响应模式测试

use puniyu_config::ReactiveMode;

#[test]
fn test_reactive_mode_serialization() {
	// 测试所有响应模式的序列化
	let modes = vec![
		(ReactiveMode::All, 0u8),
		(ReactiveMode::AtBot, 1u8),
		(ReactiveMode::Alias, 2u8),
		(ReactiveMode::AtOrAlias, 3u8),
		(ReactiveMode::Master, 4u8),
	];

	for (mode, expected_value) in modes {
		let serialized = serde_json::to_string(&mode).unwrap();
		assert_eq!(serialized, expected_value.to_string());
	}
}

#[test]
fn test_reactive_mode_deserialization() {
	// 测试从数字反序列化
	let mode: ReactiveMode = serde_json::from_str("0").unwrap();
	assert!(matches!(mode, ReactiveMode::All));

	let mode: ReactiveMode = serde_json::from_str("1").unwrap();
	assert!(matches!(mode, ReactiveMode::AtBot));

	let mode: ReactiveMode = serde_json::from_str("2").unwrap();
	assert!(matches!(mode, ReactiveMode::Alias));

	let mode: ReactiveMode = serde_json::from_str("3").unwrap();
	assert!(matches!(mode, ReactiveMode::AtOrAlias));

	let mode: ReactiveMode = serde_json::from_str("4").unwrap();
	assert!(matches!(mode, ReactiveMode::Master));
}

#[test]
fn test_reactive_mode_clone() {
	let mode = ReactiveMode::All;
	let cloned = mode.clone();

	// 验证克隆后的值相同
	let original_json = serde_json::to_string(&mode).unwrap();
	let cloned_json = serde_json::to_string(&cloned).unwrap();
	assert_eq!(original_json, cloned_json);
}

#[test]
fn test_reactive_mode_debug() {
	// 测试 Debug trait
	let mode = ReactiveMode::All;
	let debug_str = format!("{:?}", mode);
	assert!(debug_str.contains("All"));

	let mode = ReactiveMode::AtBot;
	let debug_str = format!("{:?}", mode);
	assert!(debug_str.contains("AtBot"));
}

#[test]
fn test_reactive_mode_in_config() {
	// 测试在配置中使用响应模式
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Serialize, Deserialize)]
	struct TestConfig {
		mode: ReactiveMode,
	}

	let config = TestConfig { mode: ReactiveMode::AtBot };

	let json = serde_json::to_string(&config).unwrap();
	let deserialized: TestConfig = serde_json::from_str(&json).unwrap();

	assert!(matches!(deserialized.mode, ReactiveMode::AtBot));
}

#[test]
fn test_reactive_mode_toml_serialization() {
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Serialize, Deserialize)]
	struct TestConfig {
		mode: ReactiveMode,
	}

	let config = TestConfig { mode: ReactiveMode::AtOrAlias };

	let toml_str = toml::to_string(&config).unwrap();
	assert!(toml_str.contains("mode"));

	let deserialized: TestConfig = toml::from_str(&toml_str).unwrap();
	assert!(matches!(deserialized.mode, ReactiveMode::AtOrAlias));
}

#[test]
fn test_all_reactive_modes() {
	// 确保所有模式都可以正常工作
	let modes = vec![
		ReactiveMode::All,
		ReactiveMode::AtBot,
		ReactiveMode::Alias,
		ReactiveMode::AtOrAlias,
		ReactiveMode::Master,
	];

	for mode in modes {
		// 测试克隆
		let _ = mode.clone();

		// 测试调试输出
		let _ = format!("{:?}", mode);

		// 测试序列化
		let _ = serde_json::to_string(&mode).unwrap();
	}
}

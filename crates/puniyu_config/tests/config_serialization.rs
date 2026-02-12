//! 配置序列化和反序列化测试

use puniyu_config::{app::AppConfig, bot::BotConfig, friend::FriendConfig, group::GroupConfig};
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

// 初始化测试环境
fn init_test_env() {
	use puniyu_common::APP_NAME;
	use puniyu_path::WORKING_DIR;

	// 初始化工作目录
	let _ = WORKING_DIR.set(PathBuf::from("./test_workspace"));

	// 初始化应用名称
	let _ = APP_NAME.set("puniyu_test".to_string());
}

#[test]
fn test_app_config_serialize_deserialize() {
	init_test_env();
	let config = AppConfig::default();

	// 序列化为 TOML
	let toml_str = toml::to_string(&config).expect("序列化失败");
	assert!(!toml_str.is_empty());

	// 反序列化
	let deserialized: AppConfig = toml::from_str(&toml_str).expect("反序列化失败");

	// 验证关键字段
	assert_eq!(deserialized.prefix(), config.prefix());
	assert_eq!(deserialized.masters(), config.masters());
}

#[test]
fn test_bot_config_serialize_deserialize() {
	init_test_env();
	let config = BotConfig::default();

	let toml_str = toml::to_string(&config).expect("序列化失败");
	let deserialized: BotConfig = toml::from_str(&toml_str).expect("反序列化失败");

	assert_eq!(deserialized.global().cd(), config.global().cd());
}

#[test]
fn test_group_config_serialize_deserialize() {
	init_test_env();
	let config = GroupConfig::default();

	let toml_str = toml::to_string(&config).expect("序列化失败");
	let deserialized: GroupConfig = toml::from_str(&toml_str).expect("反序列化失败");

	assert_eq!(deserialized.global().cd(), config.global().cd());
}

#[test]
fn test_friend_config_serialize_deserialize() {
	init_test_env();
	let config = FriendConfig::default();

	let toml_str = toml::to_string(&config).expect("序列化失败");
	let deserialized: FriendConfig = toml::from_str(&toml_str).expect("反序列化失败");

	assert_eq!(deserialized.global().cd(), config.global().cd());
}

#[test]
fn test_app_config_from_toml_string() {
	init_test_env();
	let toml_content = r#"
        prefix = "/"
        masters = ["admin1", "admin2"]
        
        [logger]
        level = "debug"
        
        [server]
        host = "0.0.0.0"
        port = 9000
    "#;

	let config: AppConfig = toml::from_str(toml_content).expect("解析 TOML 失败");

	assert_eq!(config.prefix(), "/");
	assert_eq!(config.masters(), &vec!["admin1".to_string(), "admin2".to_string()]);
	assert_eq!(config.logger().level(), "debug");
	assert_eq!(config.server().host(), IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
	assert_eq!(config.server().port(), 9000);
}

#[test]
fn test_bot_config_from_toml_string() {
	init_test_env();
	let toml_content = r#"
        [global]
        cd = 100
        alias = ["bot1", "bot2"]
        
        [bot.bot123]
        cd = 200
        alias = ["custom"]
    "#;

	let config: BotConfig = toml::from_str(toml_content).expect("解析 TOML 失败");

	assert_eq!(config.global().cd(), 100);
	assert_eq!(config.bot("bot123").cd(), 200);
	assert_eq!(config.bot("unknown").cd(), 100); // 回退到全局配置
}

#[test]
fn test_group_config_from_toml_string() {
	init_test_env();
	let toml_content = r#"
        [global]
        cd = 50
        user_cd = 30
        
        [group.group456]
        cd = 100
        user_cd = 60
    "#;

	let config: GroupConfig = toml::from_str(toml_content).expect("解析 TOML 失败");

	assert_eq!(config.global().cd(), 50);
	assert_eq!(config.group("group456").cd(), 100);
	assert_eq!(config.group("unknown").cd(), 50); // 回退到全局配置
}

#[test]
fn test_friend_config_from_toml_string() {
	init_test_env();
	let toml_content = r#"
        [global]
        cd = 80
        alias = ["friend"]
        
        [friend.user789]
        cd = 120
        alias = []
    "#;

	let config: FriendConfig = toml::from_str(toml_content).expect("解析 TOML 失败");

	assert_eq!(config.global().cd(), 80);
	assert_eq!(config.friend("user789").cd(), 120);
	assert_eq!(config.friend("unknown").cd(), 80); // 回退到全局配置
}

#[test]
fn test_partial_config_deserialization() {
	init_test_env();
	// 测试部分配置字段的反序列化（其他使用默认值）
	let toml_content = r##"
        prefix = "#"
    "##;

	let config: AppConfig = toml::from_str(toml_content).expect("解析部分配置失败");

	assert_eq!(config.prefix(), "#");
	assert_eq!(config.masters(), &vec!["console".to_string()]); // 默认值
}

#[test]
fn test_empty_config_deserialization() {
	init_test_env();
	// 测试空配置使用默认值
	let toml_content = "";

	let config: AppConfig = toml::from_str(toml_content).expect("解析空配置失败");

	assert_eq!(config.prefix(), "!");
	assert_eq!(config.masters(), &vec!["console".to_string()]);
}

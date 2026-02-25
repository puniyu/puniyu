//! 配置文件操作测试

use assert_fs::TempDir;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::net::{IpAddr, Ipv4Addr};

#[test]
fn test_create_config_file() {
	let temp = TempDir::new().unwrap();
	let config_file = temp.child("app.toml");

	// 创建配置文件
	config_file
		.write_str(
			r#"
        prefix = "!"
        masters = ["console"]
    "#,
		)
		.unwrap();

	// 验证文件存在
	config_file.assert(predicate::path::exists());

	// 验证文件内容
	config_file.assert(predicate::str::contains("prefix"));
	config_file.assert(predicate::str::contains("masters"));
}

#[test]
fn test_read_config_file() {
	let temp = TempDir::new().unwrap();
	let config_file = temp.child("bot.toml");

	let toml_content = r#"
        [global]
        cd = 100
        alias = ["bot"]
        
        [bot.bot001]
        cd = 200
    "#;

	config_file.write_str(toml_content).unwrap();

	// 读取并解析配置
	let content = std::fs::read_to_string(config_file.path()).unwrap();
	let config: puniyu_config::bot::BotConfig = toml::from_str(&content).unwrap();

	assert_eq!(config.global().cd(), 100);
	assert_eq!(config.bot("bot001").cd(), 200);
}

#[test]
fn test_update_config_file() {
	let temp = TempDir::new().unwrap();
	let config_file = temp.child("group.toml");

	// 初始配置
	config_file
		.write_str(
			r#"
        [global]
        cd = 50
    "#,
		)
		.unwrap();

	config_file.assert(predicate::str::contains("cd = 50"));

	// 更新配置
	config_file
		.write_str(
			r#"
        [global]
        cd = 100
    "#,
		)
		.unwrap();

	config_file.assert(predicate::str::contains("cd = 100"));
}

#[test]
fn test_multiple_config_files() {
	let temp = TempDir::new().unwrap();

	let app_config = temp.child("app.toml");
	let bot_config = temp.child("bot.toml");
	let group_config = temp.child("group.toml");
	let friend_config = temp.child("friend.toml");

	// 创建多个配置文件
	app_config.write_str(r#"prefix = "!""#).unwrap();
	bot_config
		.write_str(
			r#"[global]
cd = 0"#,
		)
		.unwrap();
	group_config
		.write_str(
			r#"[global]
cd = 0"#,
		)
		.unwrap();
	friend_config
		.write_str(
			r#"[global]
cd = 0"#,
		)
		.unwrap();

	// 验证所有文件都存在
	app_config.assert(predicate::path::exists());
	bot_config.assert(predicate::path::exists());
	group_config.assert(predicate::path::exists());
	friend_config.assert(predicate::path::exists());
}

#[test]
fn test_config_file_with_comments() {
	let temp = TempDir::new().unwrap();
	let config_file = temp.child("app.toml");

	let toml_content = r#"
        # 这是注释
        prefix = "!"
        
        # Bot主人列表
        masters = ["admin"]
        
        [logger]
        # 日志级别
        level = "info"
    "#;

	config_file.write_str(toml_content).unwrap();

	// 验证可以正确解析带注释的配置
	let content = std::fs::read_to_string(config_file.path()).unwrap();
	let config: puniyu_config::app::AppConfig = toml::from_str(&content).unwrap();

	assert_eq!(config.prefix().as_ref().map(|s| s.as_str()), Some("!"));
	assert_eq!(config.masters(), vec!["admin".to_string()]);
}

#[test]
fn test_invalid_config_file() {
	let temp = TempDir::new().unwrap();
	let config_file = temp.child("invalid.toml");

	// 写入无效的 TOML
	config_file
		.write_str(
			r#"
        invalid toml content [[[
    "#,
		)
		.unwrap();

	// 尝试解析应该失败
	let content = std::fs::read_to_string(config_file.path()).unwrap();
	let result: Result<puniyu_config::app::AppConfig, _> = toml::from_str(&content);

	assert!(result.is_err());
}

#[test]
fn test_nested_config_structure() {
	let temp = TempDir::new().unwrap();
	let config_file = temp.child("app.toml");

	let toml_content = r#"
        prefix = "/"
        
        [logger]
        level = "debug"
        
        [server]
        host = "0.0.0.0"
        port = 8080
        
        [adapter]
        name = "console"
    "#;

	config_file.write_str(toml_content).unwrap();

	let content = std::fs::read_to_string(config_file.path()).unwrap();
	let config: puniyu_config::app::AppConfig = toml::from_str(&content).unwrap();

	assert_eq!(config.prefix().as_ref().map(|s| s.as_str()), Some("/"));
	assert_eq!(config.logger().level(), "debug");
	assert_eq!(config.server().host(), IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
	assert_eq!(config.server().port(), 8080);
}

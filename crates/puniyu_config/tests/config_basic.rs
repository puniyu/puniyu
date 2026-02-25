//! 基础配置功能测试

use puniyu_config::{
	ReactiveMode, app::AppConfig, bot::BotConfig, friend::FriendConfig, group::GroupConfig,
};
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
fn test_app_config_default() {
	init_test_env();
	let config = AppConfig::default();

	// 验证默认值
	assert_eq!(config.logger().level(), "info");
	assert_eq!(config.server().host(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
	assert_eq!(config.server().port(), 33720);
	assert_eq!(config.masters(), vec!["console".to_string()]);
	assert_eq!(config.prefix().as_ref().map(|s| s.as_str()), Some("!"));
}

#[test]
fn test_bot_config_default() {
	init_test_env();
	let config = BotConfig::default();

	// 验证全局配置存在
	let global = config.global();
	assert_eq!(global.cd(), 0);
	assert!(global.alias().is_empty());
}

#[test]
fn test_group_config_default() {
	init_test_env();
	let config = GroupConfig::default();

	// 验证全局配置
	let global = config.global();
	assert_eq!(global.cd(), 0);
	assert_eq!(global.user_cd(), 0);
}

#[test]
fn test_friend_config_default() {
	init_test_env();
	let config = FriendConfig::default();

	// 验证全局配置
	let global = config.global();
	assert_eq!(global.cd(), 0);
	assert!(global.alias().is_empty());
}

#[test]
fn test_reactive_mode_values() {
	// 测试响应模式枚举值
	let _mode_all = ReactiveMode::All;
	let mode_at = ReactiveMode::AtBot;

	// 验证枚举可以被克隆和调试
	let _ = format!("{:?}", _mode_all);
	let _ = mode_at.clone();
}

#[test]
fn test_bot_config_get_specific_bot() {
	init_test_env();
	let config = BotConfig::default();

	// 获取不存在的bot配置应该返回全局配置
	let bot_option = config.bot("non_existent_bot");
	assert_eq!(bot_option.cd(), config.global().cd());
}

#[test]
fn test_group_config_get_specific_group() {
	init_test_env();
	let config = GroupConfig::default();

	// 获取不存在的群组配置应该返回全局配置
	let group_option = config.group("non_existent_group");
	assert_eq!(group_option.cd(), config.global().cd());
}

#[test]
fn test_friend_config_get_specific_friend() {
	init_test_env();
	let config = FriendConfig::default();

	// 获取不存在的好友配置应该返回全局配置
	let friend_option = config.friend("non_existent_friend");
	assert_eq!(friend_option.cd(), config.global().cd());
}

#[test]
fn test_config_list_methods() {
	init_test_env();
	let bot_config = BotConfig::default();
	let group_config = GroupConfig::default();
	let friend_config = FriendConfig::default();

	// 测试列表方法
	assert!(bot_config.list().is_empty());
	assert!(group_config.list().is_empty());
	assert!(friend_config.list().is_empty());
}

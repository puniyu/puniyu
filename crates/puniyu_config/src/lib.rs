//! # puniyu_config
//!
//! Puniyu 框架的配置管理模块，提供统一的配置读取、热重载和管理功能。
//!
//! ## 功能特性
//!
//! - **多层级配置**: 支持应用级、Bot级、群组级和好友级配置
//! - **热重载**: 自动监听配置文件变化并实时更新
//! - **类型安全**: 使用强类型配置结构，避免运行时错误
//! - **默认值**: 所有配置项都有合理的默认值
//! - **配置合并**: 支持默认配置与用户配置的智能合并
//!
//! ## 配置层级
//!
//! 1. **应用配置 (AppConfig)**: 全局应用设置，包括日志、服务器、适配器等
//! 2. **Bot配置 (BotConfig)**: 每个Bot实例的独立配置
//! 3. **群组配置 (GroupConfig)**: 群聊相关的配置，支持全局和单独群组配置
//! 4. **好友配置 (FriendConfig)**: 好友相关的配置，支持全局和单独好友配置
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use puniyu_config::{Config, init_config, start_config_watcher};
//!
//! // 初始化配置系统
//! init_config();
//!
//! // 启动配置文件监听器（可选，用于热重载）
//! start_config_watcher();
//!
//! // 获取应用配置
//! let app_config = Config::app();
//! println!("日志级别: {}", app_config.logger().level());
//!
//! // 获取Bot配置
//! let bot_config = Config::bot();
//! let bot_option = bot_config.bot("bot_id_123");
//!
//! // 获取群组配置
//! let group_config = Config::group();
//! let group_option = group_config.group("group_id_456");
//!
//! // 获取好友配置
//! let friend_config = Config::friend();
//! let friend_option = friend_config.friend("user_id_789");
//! ```
//!
//! ## 配置文件格式
//!
//! 所有配置文件使用 TOML 格式，存储在配置目录中：
//!
//! - `app.toml`: 应用配置
//! - `bot.toml`: Bot配置
//! - `group.toml`: 群组配置
//! - `friend.toml`: 好友配置
//!
//! ## 配置热重载
//!
//! 配置模块使用 `notify-debouncer-full` 监听配置文件变化，
//! 当检测到文件修改时会自动重新加载配置，无需重启应用。
//!
//! ## 响应模式
//!
//! 通过 [`ReactiveMode`] 枚举定义Bot的响应行为：
//!
//! - `All`: 响应所有消息
//! - `AtBot`: 仅响应@Bot的消息
//! - `Alias`: 仅响应使用别名的消息
//! - `AtOrAlias`: 响应@Bot或使用别名的消息
//! - `Master`: 仅响应主人的消息

pub mod app;
pub mod bot;
mod common;
pub mod friend;
pub mod group;
#[cfg(feature = "registry")]
pub mod registry;
pub mod types;

#[doc(inline)]
pub use common::ReactiveMode;

use crate::{
	app::{APP_CONFIG, AppConfig},
	bot::{BOT_CONFIG, BotConfig},
	friend::{FRIEND_CONFIG, FriendConfig},
	group::{GROUP_CONFIG, GroupConfig},
};
use notify_debouncer_full::{DebounceEventResult, new_debouncer, notify};
use puniyu_common::{merge_config, read_config};
use puniyu_error::config::Error;
use puniyu_logger::{debug, error, info};
use puniyu_path::{CONFIG_DIR, LOG_DIR};
use serde::{Deserialize, Serialize};
use std::{env, thread, time::Duration};

fn reload_config<T>(name: &str, config: &mut T) -> Result<(), Error>
where
	T: Default + for<'de> Deserialize<'de>,
{
	*config = read_config(CONFIG_DIR.as_path(), name).unwrap_or_else(|_| T::default());
	Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config;
impl Config {
	/// 获取应用配置
	///
	/// # 返回值
	///
	/// 返回应用级配置，包括日志、服务器、适配器等全局设置
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_config::Config;
	///
	/// let config = Config::app();
	/// println!("日志级别: {}", config.logger().level());
	/// println!("服务器端口: {}", config.server().port());
	/// ```
	pub fn app() -> AppConfig {
		AppConfig::get()
	}

	/// 获取好友配置
	///
	/// # 返回值
	///
	/// 返回好友级配置，包括全局好友配置和特定好友配置
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_config::Config;
	///
	/// let config = Config::friend();
	/// let friend_option = config.friend("user_123");
	/// println!("好友 CD: {}", friend_option.cd());
	/// ```
	pub fn friend() -> FriendConfig {
		FriendConfig::get()
	}

	/// 获取群组配置
	///
	/// # 返回值
	///
	/// 返回群组级配置，包括全局群组配置和特定群组配置
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_config::Config;
	///
	/// let config = Config::group();
	/// let group_option = config.group("group_456");
	/// println!("群组 CD: {}", group_option.cd());
	/// ```
	pub fn group() -> GroupConfig {
		GroupConfig::get()
	}

	/// 获取 Bot 配置
	///
	/// # 返回值
	///
	/// 返回 Bot 级配置，包括全局 Bot 配置和特定 Bot 配置
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_config::Config;
	///
	/// let config = Config::bot();
	/// let bot_option = config.bot("bot_001");
	/// println!("Bot 别名: {:?}", bot_option.alias());
	/// ```
	pub fn bot() -> BotConfig {
		BotConfig::get()
	}
}

/// 初始化配置系统
///
/// 该函数会执行以下操作：
/// 1. 创建配置目录（如果不存在）
/// 2. 创建日志目录（如果不存在）
/// 3. 合并默认配置与用户配置
/// 4. 初始化环境变量
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_config::init_config;
///
/// // 在应用启动时调用
/// init_config();
/// ```
///
/// # 注意
///
/// 该函数应该在应用启动时调用一次，通常在 `main` 函数的开始处。
pub fn init_config() {
	if !CONFIG_DIR.as_path().exists() {
		std::fs::create_dir_all(CONFIG_DIR.as_path())
			.unwrap_or_else(|_| error!("[配置文件] 初始化配置文件失败"));
	}
	if !LOG_DIR.as_path().exists() {
		std::fs::create_dir_all(LOG_DIR.as_path())
			.unwrap_or_else(|_| error!("[配置文件] 初始化日志目录失败"));
	}
	merge_config(CONFIG_DIR.as_path(), "app", &AppConfig::default(), &AppConfig::get())
		.unwrap_or_else(|e| {
			error!("[配置文件] 合并APP配置失败: {}", e);
		});
	merge_config(CONFIG_DIR.as_path(), "group", &GroupConfig::default(), &GroupConfig::get())
		.unwrap_or_else(|e| {
			error!("[配置文件] 合并Group配置失败: {}", e);
		});
	merge_config(CONFIG_DIR.as_path(), "friend", &FriendConfig::default(), &FriendConfig::get())
		.unwrap_or_else(|e| {
			error!("[配置文件] 合并Friend配置失败: {}", e);
		});
	merge_config(CONFIG_DIR.as_path(), "bot", &BotConfig::default(), &BotConfig::get())
		.unwrap_or_else(|e| {
			error!("[配置文件] 合并Bot配置失败: {}", e);
		});
	init_env();
}

/// 启动配置文件监听器
///
/// 该函数会在后台线程中启动一个文件监听器，自动监听配置文件的变化。
/// 当配置文件被修改时，会自动重新加载配置，实现热重载功能。
///
/// # 监听的文件
///
/// - `app.toml` - 应用配置
/// - `bot.toml` - Bot 配置
/// - `group.toml` - 群组配置
/// - `friend.toml` - 好友配置
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_config::{init_config, start_config_watcher};
///
/// // 初始化配置
/// init_config();
///
/// // 启动配置监听器（可选）
/// start_config_watcher();
///
/// // 现在配置文件的任何修改都会自动生效
/// ```
///
/// # 注意
///
/// - 该函数会创建一个后台线程，不会阻塞主线程
/// - 配置变更会有约 2 秒的防抖延迟
/// - 如果不需要热重载功能，可以不调用此函数
pub fn start_config_watcher() {
	thread::spawn(|| {
		debug!("[Config] 配置文件监听器已启动");

		let mut debouncer =
			new_debouncer(Duration::from_secs(2), None, |res: DebounceEventResult| match res {
				Ok(events) => {
					for event in events.iter() {
						if !matches!(
							event.event.kind,
							notify::EventKind::Modify(_)
								| notify::EventKind::Create(_)
								| notify::EventKind::Remove(_)
						) {
							continue;
						}

						for path in event.event.paths.iter() {
							info!("[Config] 文件变更: {}", path.display());

							if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
								match file_name {
									"app.toml" => {
										reload_config(
											"app",
											&mut *APP_CONFIG
												.write()
												.expect("Failed to acquire app config write lock"),
										)
										.expect("Failed to reload app config");
									}
									"bot.toml" => {
										reload_config(
											"bot",
											&mut *BOT_CONFIG
												.write()
												.expect("Failed to acquire bot config write lock"),
										)
										.expect("Failed to reload bot config");
									}
									"group.toml" => {
										reload_config(
											"group",
											&mut *GROUP_CONFIG.write().expect(
												"Failed to acquire group config write lock",
											),
										)
										.expect("Failed to reload group config");
									}
									"friend.toml" => {
										reload_config(
											"friend",
											&mut *FRIEND_CONFIG.write().expect(
												"Failed to acquire friend config write lock",
											),
										)
										.expect("Failed to reload friend config");
									}
									_ => {}
								}
								#[cfg(feature = "registry")]
								{
									if registry::ConfigRegistry::all()
										.iter()
										.any(|c| c.path == *path) && let Some(name) =
										path.file_stem().and_then(|s| s.to_str())
										&& let Some(dir) = path.parent()
										&& let Ok(value) = read_config::<toml::Value>(dir, name)
									{
										registry::ConfigRegistry::update(
											path.to_path_buf(),
											value.clone(),
										)
										.expect("Failed to update config");
										debug!("[Config] 更新配置: {:#?}", value.clone());
									}
								}
							}
						}
					}
				}
				Err(e) => error!("[Config] 监听错误: {:?}", e),
			})
			.expect("Failed to create config file watcher");

		debouncer
			.watch(CONFIG_DIR.as_path(), notify::RecursiveMode::Recursive)
			.expect("Failed to start watching config directory");

		thread::park();
	});
}
fn init_env() {
	let config = Config::app();
	if env::var("LOGGER_LEVEL").is_err() {
		let config = config.logger();
		let level = config.level();
		unsafe {
			env::set_var("LOGGER_LEVEL", level);
		}
	}
	if env::var("HTTP_HOST").is_err() {
		unsafe {
			env::set_var("HTTP_HOST", Config::app().server().host().to_string());
		}
	}
	if env::var("HTTP_PORT").is_err() {
		unsafe {
			env::set_var("HTTP_PORT", Config::app().server().port().to_string());
		}
	}
}

mod app;
#[doc(inline)]
pub use app::AppConfig;
mod bot;
#[doc(inline)]
pub use bot::BotConfig;
mod friend;
#[doc(inline)]
pub use friend::FriendConfig;
mod group;
#[doc(inline)]
pub use group::GroupConfig;
mod types;
#[doc(inline)]
pub use types::*;
mod config;
mod registry;

pub use registry::ConfigRegistry;

use puniyu_common::merge_config;
use puniyu_logger::{debug, error, info};
use puniyu_path::{config_dir, log_dir};

/// 配置 trait
///
/// 定义配置的基本接口，用于外部包实现自定义配置。
pub trait Config: Send + Sync + 'static {
	fn config(&self) -> ConfigInfo;
}

impl PartialEq for dyn Config {
	fn eq(&self, other: &Self) -> bool {
		self.config() == other.config()
	}
}

#[inline]
pub fn app_config() -> AppConfig {
	AppConfig::get()
}

#[inline]
pub fn bot_config() -> BotConfig {
	BotConfig::get()
}

#[inline]
pub fn friend_config() -> FriendConfig {
	FriendConfig::get()
}

#[inline]
pub fn group_config() -> GroupConfig {
	GroupConfig::get()
}

pub fn init() {
	if !config_dir().as_path().exists() {
		std::fs::create_dir_all(config_dir().as_path())
			.unwrap_or_else(|_| error!("[Config] Failed to initialize config directory"));
	}
	if !log_dir().as_path().exists() {
		std::fs::create_dir_all(log_dir().as_path())
			.unwrap_or_else(|_| error!("[Config] Failed to initialize log directory"));
	}

	merge_config(config_dir().as_path(), "app", &AppConfig::default(), &AppConfig::get())
		.unwrap_or_else(|e| {
			error!("[Config] Failed to merge app config: {}", e);
		});
	merge_config(config_dir().as_path(), "group", &GroupConfig::default(), &GroupConfig::get())
		.unwrap_or_else(|e| {
			error!("[Config] Failed to merge group config: {}", e);
		});
	merge_config(config_dir().as_path(), "friend", &FriendConfig::default(), &FriendConfig::get())
		.unwrap_or_else(|e| {
			error!("[Config] Failed to merge friend config: {}", e);
		});
	merge_config(config_dir().as_path(), "bot", &BotConfig::default(), &BotConfig::get())
		.unwrap_or_else(|e| {
			error!("[Config] Failed to merge bot config: {}", e);
		});

	let app_config = AppConfig::get();
	if let Err(e) = ConfigRegistry::register(app_config.config()) {
		error!("[Config] Failed to register app config: {}", e);
	} else {
		info!("[Config] App config registered");
	}

	let bot_config = BotConfig::get();
	if let Err(e) = ConfigRegistry::register(bot_config.config()) {
		error!("[Config] Failed to register bot config: {}", e);
	} else {
		debug!("[Config] Bot config registered");
	}

	let group_config = GroupConfig::get();
	if let Err(e) = ConfigRegistry::register(group_config.config()) {
		error!("[Config] Failed to register group config: {}", e);
	} else {
		debug!("[Config] Group config registered");
	}

	let friend_config = FriendConfig::get();
	if let Err(e) = ConfigRegistry::register(friend_config.config()) {
		error!("[Config] Failed to register friend config: {}", e);
	} else {
		debug!("[Config] Friend config registered");
	}
	config::start_config_watcher();
}

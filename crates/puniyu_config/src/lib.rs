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

#[doc(inline)]
pub use puniyu_config_core::{ConfigInfo, ConfigId};
#[doc(inline)]
pub use puniyu_config_core::Config;

#[doc(inline)]
#[cfg(feature = "registry")]
pub use puniyu_config_core::ConfigRegistry;


use puniyu_common::{merge_config};
use puniyu_logger::{error};
use puniyu_path::{CONFIG_DIR, LOG_DIR};


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

pub fn group_config() -> GroupConfig {
	GroupConfig::get()
}

pub fn init() {
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
	#[cfg(feature = "watcher")]
	config::start_config_watcher();
}


use std::{path::PathBuf, sync::LazyLock};

use crate::Config;
use crate::{CommandConfig, ListConfig, LoggerConfig, ServerConfig, common::read_config};
use serde::{Deserialize, Serialize};

const NAME: &str = "app";

static CONFIG_PATH: LazyLock<PathBuf> =
	LazyLock::new(|| puniyu_path::config_dir().join(NAME).with_extension("toml"));

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AppConfig {
	/// 服务器配置
	///
	/// 包括服务器主机地址和端口号
	#[serde(default)]
	server: ServerConfig,

	/// 适配器配置
	///
	/// 控制启用哪些适配器
	#[serde(default)]
	adapter: ListConfig,

	/// 插件配置
	///
	/// 控制启用哪些插件
	#[serde(default)]
	plugin: ListConfig,

	/// 应用级群组配置
	///
	/// 包含群聊黑白名单等全局设置
	#[serde(default)]
	group: ListConfig,

	/// 应用级好友配置
	///
	/// 包含好友黑白名单等全局设置
	#[serde(default)]
	friend: ListConfig,

	/// 命令配置
	///
	/// 包含配置命令前缀
	#[serde(default)]
	command: CommandConfig,

	/// 日志配置
	///
	/// 控制文件日志、日志等级和日志文件保留天数
	#[serde(default)]
	logger: LoggerConfig,
}

impl AppConfig {
	pub fn get() -> Self {
		use crate::ConfigRegistry;
		ConfigRegistry::get(CONFIG_PATH.as_path())
			.unwrap_or_else(|| read_config(CONFIG_PATH.as_path()).unwrap_or_default())
	}
	pub fn server(&self) -> ServerConfig {
		self.server.clone()
	}
	pub fn adapter(&self) -> ListConfig {
		self.adapter.clone()
	}
	pub fn plugin(&self) -> ListConfig {
		self.plugin.clone()
	}
	pub fn group(&self) -> ListConfig {
		self.group.clone()
	}
	pub fn friend(&self) -> ListConfig {
		self.friend.clone()
	}
	pub fn command(&self) -> CommandConfig {
		self.command.clone()
	}
	pub fn logger(&self) -> LoggerConfig {
		self.logger.clone()
	}
}

impl Config for AppConfig {
	fn name(&self) -> &str {
		NAME
	}
	fn path(&self) -> PathBuf {
		puniyu_path::config_dir()
	}

	#[allow(clippy::unwrap_used)]
	#[inline]
	fn value(&self) -> toml::Value {
		toml::Value::try_from(self).unwrap()
	}
}

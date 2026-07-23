use std::path::{Path, PathBuf};

use crate::{
	CommandConfig, ListConfig, LoggerConfig, MasterConfig, ServerConfig,
	common::read_config,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AppConfig {
	/// 服务器配置
	#[serde(default)]
	server: ServerConfig,

	/// 适配器配置
	#[serde(default)]
	adapter: ListConfig,

	/// 插件配置
	#[serde(default)]
	plugin: ListConfig,

	/// 应用级群组配置
	#[serde(default)]
	group: ListConfig,

	/// 应用级好友配置
	#[serde(default)]
	friend: ListConfig,

	/// 命令配置
	#[serde(default)]
	command: CommandConfig,

	/// Bot Master 权限配置
	#[serde(default)]
	master: MasterConfig,

	/// 日志配置
	#[serde(default)]
	logger: LoggerConfig,
	#[serde(skip)]
	path: PathBuf,
}

impl AppConfig {
	pub fn from_path(path: impl AsRef<Path>) -> Self {
		let mut config: Self = read_config(&path).unwrap_or_default();
		config.path = path.as_ref().to_path_buf();
		config
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
	pub fn master(&self) -> MasterConfig {
		self.master.clone()
	}
	pub fn logger(&self) -> LoggerConfig {
		self.logger.clone()
	}
}

impl crate::Config for AppConfig {
	fn name(&self) -> &str {
		"app"
	}

	fn path(&self) -> PathBuf {
		self.path.clone()
	}

	fn to_value(&self) -> toml::Value {
		toml::Value::try_from(self).expect("AppConfig serialization failed")
	}
}

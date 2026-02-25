mod adapter;
mod friend;
mod group;
mod logger;
mod server;

pub use adapter::AdapterConfig;
pub use friend::FriendConfig;
pub use group::GroupConfig;
pub use logger::LoggerConfig;
use puniyu_common::read_config;
use puniyu_path::CONFIG_DIR;
use serde::{Deserialize, Serialize};
pub use server::ServerConfig;
use std::sync::{Arc, LazyLock, RwLock};

pub(crate) static APP_CONFIG: LazyLock<Arc<RwLock<AppConfig>>> = LazyLock::new(|| {
	Arc::new(RwLock::new(read_config::<AppConfig>(CONFIG_DIR.as_path(), "app").unwrap_or_default()))
});

fn default_master() -> Vec<String> {
	vec!["console".to_string()]
}

fn default_prefix() -> Option<String> {
	Some(String::from("!"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
	/// 日志配置
	///
	/// 包括日志级别、文件记录、保留天数等设置
	#[serde(default)]
	logger: LoggerConfig,

	/// 服务器配置
	///
	/// 包括服务器主机地址和端口号
	#[serde(default)]
	server: ServerConfig,

	/// 适配器配置
	///
	/// 控制启用哪些适配器（控制台、服务器等）
	#[serde(default)]
	adapter: AdapterConfig,

	/// 应用级群组配置
	///
	/// 包含群聊黑白名单等全局设置
	#[serde(default)]
	group: GroupConfig,

	/// 应用级好友配置
	///
	/// 包含好友黑白名单等全局设置
	#[serde(default)]
	friend: FriendConfig,

	/// Bot 主人列表
	///
	/// 定义哪些用户是 Bot 的主人，拥有最高权限
	#[serde(default = "default_master")]
	masters: Vec<String>,

	/// 全局命令前缀
	///
	/// 用于识别命令的前缀字符，默认为 "!"
	#[serde(default = "default_prefix")]
	prefix: Option<String>,
}

impl Default for AppConfig {
	#[inline]
	fn default() -> Self {
		Self {
			logger: LoggerConfig::default(),
			server: ServerConfig::default(),
			adapter: AdapterConfig::default(),
			group: GroupConfig::default(),
			friend: FriendConfig::default(),
			masters: default_master(),
			prefix: default_prefix(),
		}
	}
}

impl AppConfig {
	/// 获取应用配置实例
	///
	/// # 返回值
	///
	/// 返回当前的应用配置副本
	pub fn get() -> Self {
		APP_CONFIG.read().expect("Failed to read app config").clone()
	}

	/// 获取日志配置
	///
	/// # 返回值
	///
	/// 返回日志配置的引用，包括日志级别、文件记录等设置
	pub fn logger(&self) -> &LoggerConfig {
		&self.logger
	}

	/// 获取服务器配置
	///
	/// # 返回值
	///
	/// 返回服务器配置的引用，包括主机地址和端口号
	pub fn server(&self) -> &ServerConfig {
		&self.server
	}

	/// 获取适配器配置
	///
	/// # 返回值
	///
	/// 返回适配器配置的引用，控制启用哪些适配器
	pub fn adapter(&self) -> &AdapterConfig {
		&self.adapter
	}

	/// 获取 Bot 主人列表
	///
	/// # 返回值
	///
	/// 返回主人用户 ID 列表的引用
	pub fn masters(&self) -> Vec<String> {
		self.masters.clone()
	}

	/// 获取应用级群组配置
	///
	/// # 返回值
	///
	/// 返回应用级群组配置的引用，包含群聊黑白名单等全局设置
	pub fn group(&self) -> &GroupConfig {
		&self.group
	}

	/// 获取应用级好友配置
	///
	/// # 返回值
	///
	/// 返回应用级好友配置的引用，包含好友黑白名单等全局设置
	pub fn friend(&self) -> &FriendConfig {
		&self.friend
	}

	/// 获取全局命令前缀
	///
	/// # 返回值
	///
	/// 返回命令前缀字符串的引用，默认为 "!", 为空时返回为[`None`]
	pub fn prefix(&self) -> Option<String> {
		self.prefix.clone()
	}
}

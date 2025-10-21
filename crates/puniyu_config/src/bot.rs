use puniyu_common::path::CONFIG_DIR;
use puniyu_common::toml;
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	sync::{Arc, LazyLock, RwLock},
};

pub(crate) static BOT_CONFIG: LazyLock<Arc<RwLock<BotConfig>>> = LazyLock::new(|| {
	Arc::new(RwLock::new(toml::read_config(CONFIG_DIR.as_path(), "bot").unwrap_or_default()))
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotConfigFile {
	/// 全局cd冷却时间
	#[serde(default = "default_bot_cd")]
	pub cd: u16,
	/// 用户cd冷却时间
	#[serde(default = "default_bot_user_cd")]
	pub user_cd: u16,
}

impl Default for BotConfigFile {
	fn default() -> Self {
		Self { cd: default_bot_cd(), user_cd: default_bot_user_cd() }
	}
}

impl BotConfigFile {
	pub fn cd(&self) -> u16 {
		self.cd
	}
	pub fn user_cd(&self) -> u16 {
		self.user_cd
	}
}

fn default_bot_cd() -> u16 {
	0
}

fn default_bot_user_cd() -> u16 {
	0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotConfig {
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	bot: HashMap<String, BotConfigFile>,
	/// 主人列表
	#[serde(default)]
	masters: Vec<String>,
}

impl Default for BotConfig {
	fn default() -> Self {
		Self { bot: HashMap::new(), masters: vec![String::from("console")] }
	}
}

impl BotConfig {
	/// 根据bot ID获取bot配置
	///
	/// # 参数
	///
	/// `bot_id` - bot ID
	///
	/// # 返回值
	///
	/// 返回对应bot的配置，如果找不到则返回默认配置
	pub fn get() -> Self {
		BOT_CONFIG.read().unwrap().clone()
	}

	/// 获取bot配置
	///
	/// # 参数
	///
	/// `bot_id` - bot ID
	///
	/// # 返回值
	///
	/// 返回对应bot的配置，如果找不到则返回默认配置
	pub fn bot(&self, bot_id: &str) -> BotConfigFile {
		let config = BOT_CONFIG.read().unwrap();
		config.bot.get(bot_id).cloned().unwrap_or_default()
	}

	pub fn masters(&self) -> Vec<String> {
		let config = BOT_CONFIG.read().unwrap();
		config.masters.clone()
	}
}

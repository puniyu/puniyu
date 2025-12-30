mod option;

use option::BotOption;
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BotConfig {
	#[serde(default)]
	global: BotOption,
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	bot: HashMap<String, BotOption>,
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

	/// 获取全局配置
	pub fn global(&self) -> &BotOption {
		&self.global
	}

	/// 获取bot配置
	///
	/// # 参数
	///
	/// `bot_id` - bot ID
	///
	/// # 返回值
	///
	/// 返回对应bot的配置，如果找不到则返回全局配置
	pub fn bot(&self, bot_id: &str) -> &BotOption {
		self.bot.get(bot_id).unwrap_or(&self.global)
	}

	/// 获取所有bot配置
	pub fn list(&self) -> Vec<&BotOption> {
		self.bot.values().collect()
	}
}

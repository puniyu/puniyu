pub(crate) mod option;

use puniyu_common::path::CONFIG_DIR;
use puniyu_common::toml;
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	sync::{Arc, LazyLock, RwLock},
};
use option::BotOption;

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

	/// 获取bot配置
	///
	/// # 参数
	///
	/// `bot_id` - bot ID
	///
	/// # 返回值
	///
	/// 返回对应bot的配置，如果找不到则返回默认配置
	pub fn bot(&self, bot_id: &str) -> BotOption {
		let config = BOT_CONFIG.read().unwrap();
		config.bot.get(bot_id).cloned().unwrap_or_default()
	}
}

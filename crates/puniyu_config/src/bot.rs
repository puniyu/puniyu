use puniyu_common::read_config;
use puniyu_path::CONFIG_DIR;
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	sync::{Arc, LazyLock, RwLock},
};
use crate::BotOption;

pub(crate) static BOT_CONFIG: LazyLock<Arc<RwLock<BotConfig>>> = LazyLock::new(|| {
	Arc::new(RwLock::new(read_config(CONFIG_DIR.as_path(), "bot").unwrap_or_default()))
});

/// Bot 配置结构
///
/// 管理所有 Bot 实例的配置，支持全局配置和单独 Bot 配置。
///
/// # 配置文件示例
///
/// ```toml
/// [global]
/// cd = 0
/// mode = 0
/// alias = []
///
/// [bot.bot_001]
/// cd = 5
/// mode = 1
/// alias = ["小助手", "bot"]
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BotConfig {
	/// 全局 Bot 配置
	///
	/// 作为所有 Bot 的默认配置
	#[serde(default)]
	global: BotOption,

	/// 特定 Bot 配置映射
	///
	/// 键为 Bot ID，值为该 Bot 的特定配置
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	bot: HashMap<String, BotOption>,
}

impl BotConfig {
	/// 获取 Bot 配置实例
	///
	/// # 返回值
	///
	/// 返回当前的 Bot 配置副本
	pub fn get() -> Self {
		BOT_CONFIG.read().expect("Failed to acquire lock").clone()
	}

	/// 获取全局 Bot 配置
	///
	/// # 返回值
	///
	/// 返回全局 Bot 配置的引用，作为所有 Bot 的默认配置
	pub fn global(&self) -> &BotOption {
		&self.global
	}

	/// 获取指定 Bot 的配置
	///
	/// # 参数
	///
	/// - `bot_id`: Bot 的唯一标识符
	///
	/// # 返回值
	///
	/// 返回对应 Bot 的配置。特定 Bot 配置会自动继承全局配置，
	/// 只有显式设置的字段会覆盖全局配置
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_config::Config;
	///
	/// let bot_config = Config::bot();
	/// // 如果 bot_001 只设置了 cd，其他字段会继承全局配置
	/// let bot_option = bot_config.bot("bot_001");
	/// println!("Bot CD: {}", bot_option.cd());
	/// ```
	pub fn bot(&self, bot_id: &str) -> BotOption {
		self.bot
			.get(bot_id)
			.map(|specific| specific.merge_with(&self.global))
			.unwrap_or_else(|| self.global.clone())
	}

	/// 获取所有 Bot 配置列表
	///
	/// # 返回值
	///
	/// 返回包含所有 Bot ID 和对应配置的 HashMap，每个配置都已与全局配置合并
	pub fn list(&self) -> HashMap<String, BotOption> {
		self.bot
			.iter()
			.map(|(id, specific)| (id.clone(), specific.merge_with(&self.global)))
			.collect()
	}
}

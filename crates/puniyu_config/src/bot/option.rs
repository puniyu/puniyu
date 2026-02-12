use crate::common::ReactiveMode;
use serde::{Deserialize, Serialize};

/// Bot 配置选项
///
/// 定义单个 Bot 实例的配置参数。
///
/// # 配置继承
///
/// 特定 Bot 配置会继承全局配置。如果某个字段未设置（`None`），
/// 则使用全局配置中的对应值。
///
/// # 配置项
///
/// - `cd`: 消息冷却时间（秒），防止频繁响应
/// - `mode`: 响应模式，控制 Bot 响应哪些消息
/// - `alias`: Bot 别名列表，用于命令识别
///
/// # 示例
///
/// ```toml
/// [global]
/// cd = 0
/// mode = 0
/// alias = []
///
/// [bot.bot_001]
/// # 只覆盖 cd 和 mode，alias 继承全局配置
/// cd = 5
/// mode = 1
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotOption {
	/// 消息冷却时间（秒）
	///
	/// 设置 Bot 响应消息的最小间隔时间，防止频繁响应
	/// 如果未设置，继承全局配置
	#[serde(skip_serializing_if = "Option::is_none")]
	cd: Option<u64>,

	/// 响应模式
	///
	/// 控制 Bot 响应哪些类型的消息
	/// 如果未设置，继承全局配置
	#[serde(skip_serializing_if = "Option::is_none")]
	mode: Option<ReactiveMode>,

	/// Bot 别名列表
	///
	/// 用于命令识别，当消息以别名开头时会被识别为命令
	/// 如果未设置，继承全局配置
	#[serde(skip_serializing_if = "Option::is_none")]
	alias: Option<Vec<String>>,
}

impl Default for BotOption {
	#[inline]
	fn default() -> Self {
		Self { cd: Some(0), mode: Some(ReactiveMode::All), alias: Some(Vec::new()) }
	}
}

impl BotOption {
	/// 与全局配置合并，返回完整的配置
	///
	/// # 参数
	///
	/// - `global`: 全局配置
	///
	/// # 返回值
	///
	/// 返回合并后的配置，特定配置会覆盖全局配置
	pub fn merge_with(&self, global: &BotOption) -> BotOption {
		BotOption {
			cd: self.cd.or(global.cd),
			mode: self.mode.clone().or(global.mode.clone()),
			alias: self.alias.clone().or(global.alias.clone()),
		}
	}

	/// 获取消息冷却时间
	///
	/// # 返回值
	///
	/// 返回冷却时间（秒），0 表示无冷却
	pub fn cd(&self) -> u64 {
		self.cd.unwrap_or(0)
	}

	/// 获取 Bot 别名列表
	///
	/// # 返回值
	///
	/// 返回别名列表的副本
	pub fn alias(&self) -> Vec<String> {
		self.alias.clone().unwrap_or_default()
	}

	/// 获取响应模式
	///
	/// # 返回值
	///
	/// 返回当前的响应模式
	pub fn mode(&self) -> ReactiveMode {
		self.mode.clone().unwrap_or(ReactiveMode::All)
	}
}

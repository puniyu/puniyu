use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ReactiveMode {
	/// 所有人
	All = 0,
	/// 仅艾特Bot
	AtBot = 1,
	/// 仅别名
	Alias = 2,
	/// 别名或艾特
	AtOrAlias = 3,
	/// 仅主人
	Master = 4,
}

/// TODO: Bot中的禁用/启用插件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotOption {
	/// 好友消息cd冷却时间
	#[serde(default = "default_bot_cd")]
	cd: u16,
	#[serde(default = "default_reactive_mode")]
	/// 响应模式
	mode: ReactiveMode,
	/// bot别名
	#[serde(default = "default_alias")]
	alias: Vec<String>,
}

impl Default for BotOption {
	fn default() -> Self {
		Self {
			cd: default_bot_cd(),
			mode: default_reactive_mode(),
			alias: default_alias(),
		}
	}
}

fn default_reactive_mode() -> ReactiveMode {
	ReactiveMode::All
}

fn default_alias() -> Vec<String> {
	Vec::new()
}

impl BotOption {
	/// 获取好友消息cd冷却时间
	pub fn cd(&self) -> u16 {
		self.cd
	}
	
	/// 获取Bot别名
	pub fn alias(&self) -> Vec<String> {
		self.alias.clone()
	}
	/// 获取响应模式
	pub fn mode(&self) -> ReactiveMode {
		self.mode.clone()
	}
}

fn default_bot_cd() -> u16 {
	0
}

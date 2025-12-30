use crate::common::ReactiveMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendOption {
	/// 好友消息cd冷却时间
	#[serde(default = "default_bot_cd")]
	cd: u64,
	#[serde(default = "default_reactive_mode")]
	/// 响应模式
	mode: ReactiveMode,
	/// bot别名
	#[serde(default = "default_alias")]
	alias: Vec<String>,
}

impl Default for FriendOption {
	#[inline]
	fn default() -> Self {
		Self { cd: default_bot_cd(), mode: default_reactive_mode(), alias: default_alias() }
	}
}

fn default_reactive_mode() -> ReactiveMode {
	ReactiveMode::All
}

fn default_alias() -> Vec<String> {
	Vec::new()
}

fn default_bot_cd() -> u64 {
	0
}

impl FriendOption {
	/// 获取消息cd冷却时间
	pub fn cd(&self) -> u64 {
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

use crate::common::ReactiveMode;
use serde::{Deserialize, Serialize};

/// TODO: 群组中的禁用/启用插件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupOption {
	/// 群组cd冷却时间
	#[serde(default = "default_group_cd")]
	cd: u64,
	/// 用户cd冷却时间
	#[serde(default = "default_group_user_cd")]
	user_cd: u64,
	#[serde(default = "default_reactive_mode")]
	/// 响应模式
	mode: ReactiveMode,
	/// bot别名
	#[serde(default = "default_alias")]
	alias: Vec<String>,
}

impl Default for GroupOption {
	#[inline]
	fn default() -> Self {
		Self {
			cd: default_group_cd(),
			user_cd: default_group_user_cd(),
			mode: default_reactive_mode(),
			alias: default_alias(),
		}
	}
}

fn default_group_cd() -> u64 {
	0
}

fn default_group_user_cd() -> u64 {
	0
}

fn default_reactive_mode() -> ReactiveMode {
	ReactiveMode::All
}

fn default_alias() -> Vec<String> {
	Vec::new()
}

impl GroupOption {
	/// 获取群组cd冷却时间
	pub fn cd(&self) -> u64 {
		self.cd
	}
	/// 获取用户cd冷却时间
	pub fn user_cd(&self) -> u64 {
		self.user_cd
	}
	/// 获取响应模式
	pub fn mode(&self) -> ReactiveMode {
		self.mode.clone()
	}
	/// 获取bot别名
	pub fn alias(&self) -> Vec<String> {
		self.alias.clone()
	}
}

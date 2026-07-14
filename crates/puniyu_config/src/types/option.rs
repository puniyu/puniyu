use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smol_str::SmolStr;
use std::time::Duration;
use strum::{Display, EnumString, IntoStaticStr};

const fn default_cd() -> u64 {
	0
}

/// Bot 响应模式枚举
///
/// 定义 Bot 在不同场景下的响应行为。
///
/// # 示例
///
/// ```rust
/// use puniyu_config::ReactiveMode;
///
/// let mode = ReactiveMode::AtBot;
/// match mode {
///     ReactiveMode::All => println!("响应所有消息"),
///     ReactiveMode::AtBot => println!("仅响应 @ Bot 的消息"),
///     ReactiveMode::Alias => println!("仅响应使用别名的消息"),
///     ReactiveMode::AtOrAlias => println!("响应 @ Bot 或使用别名的消息"),
///     ReactiveMode::Master => println!("仅响应主人的消息"),
/// }
/// ```
#[derive(
	Debug,
	Default,
	Clone,
	Copy,
	PartialEq,
	Eq,
	Hash,
	EnumString,
	Display,
	IntoStaticStr,
	Serialize_repr,
	Deserialize_repr,
)]
#[repr(u8)]
pub enum ReactiveMode {
	/// 响应所有消息
	///
	/// Bot 会响应所有接收到的消息，不做任何过滤
	#[default]
	All = 0,

	/// 仅响应 @ Bot 的消息
	///
	/// 只有当消息中包含 @ Bot 时才会响应
	AtBot = 1,

	/// 仅响应使用别名的消息
	///
	/// 只有当消息以配置的别名开头时才会响应
	Alias = 2,

	/// 响应 @ Bot 或使用别名的消息
	///
	/// 当消息包含 @ Bot 或以别名开头时都会响应
	AtOrAlias = 3,

	/// 仅响应主人的消息
	///
	/// 只响应配置中定义的主人发送的消息
	Master = 4,
}

/// Bot 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionConfig {
	/// 消息冷却时间（毫秒）
	#[serde(default, deserialize_with = "deserialize_cd", serialize_with = "serialize_cd")]
	pub cd: Duration,
	/// 响应模式
	pub mode: ReactiveMode,
	/// Bot 别名列表
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub alias: Vec<SmolStr>,
}

impl Default for OptionConfig {
	#[inline]
	fn default() -> Self {
		Self {
			cd: Duration::from_millis(default_cd()),
			mode: Default::default(),
			alias: Default::default(),
		}
	}
}

fn deserialize_cd<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
	let ms = u64::deserialize(d)?;
	Ok(Duration::from_millis(ms))
}

fn serialize_cd<S: serde::Serializer>(v: &Duration, s: S) -> Result<S::Ok, S::Error> {
	(v.as_millis() as u64).serialize(s)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct OptionConfigRaw {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub cd: Option<u64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub mode: Option<ReactiveMode>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub alias: Option<Vec<SmolStr>>,
}

impl crate::common::MergeWith for OptionConfigRaw {
	type Value = OptionConfig;

	fn merge_with(&self, global: &OptionConfig) -> Self::Value {
		OptionConfig {
			cd: self.cd.map(Duration::from_millis).unwrap_or(global.cd),
			mode: self.mode.unwrap_or(global.mode),
			alias: self.alias.clone().unwrap_or(global.alias.clone()),
		}
	}
}

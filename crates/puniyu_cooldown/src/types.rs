use smol_str::SmolStr;
use std::fmt::{Display, Formatter};
use std::time::Duration;

/// 消息冷却作用域。
///
/// 不同作用域之间精确匹配，不会自动进行层级联动。
///
/// # 示例
///
/// ```text
/// use puniyu_cooldown::CooldownScope;
///
/// // 全局冷却
/// let global = CooldownScope::Global;
///
/// // 机器人级别冷却
/// let bot = CooldownScope::bot("123456");
///
/// // 好友级别冷却
/// let friend = CooldownScope::friend("123456", "789012");
///
/// // 群组级别冷却
/// let group = CooldownScope::group("123456", "456789");
///
/// // 群成员级别冷却
/// let member = CooldownScope::group_member("123456", "456789", "789012");
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum CooldownScope {
	/// 全局冷却
	///
	/// 所有用户共享的冷却时间，适用于全局限制的功能。
	#[default]
	Global,

	/// 机器人级别冷却
	///
	/// 特定机器人的所有用户共享的冷却时间。
	Bot {
		/// 机器人 ID
		bot_id: SmolStr,
	},

	/// 好友级别冷却
	///
	/// 特定机器人的特定好友的冷却时间。
	Friend {
		/// 机器人 ID
		bot_id: SmolStr,
		/// 用户 ID
		user_id: SmolStr,
	},

	/// 群组级别冷却
	///
	/// 特定机器人的特定群组的冷却时间，群内所有成员共享。
	Group {
		/// 机器人 ID
		bot_id: SmolStr,
		/// 群组 ID
		group_id: SmolStr,
	},

	/// 群成员级别冷却
	///
	/// 特定群组中特定成员的冷却时间，最细粒度的冷却控制。
	GroupMember {
		/// 机器人 ID
		bot_id: SmolStr,
		/// 群组 ID
		group_id: SmolStr,
		/// 用户 ID
		user_id: SmolStr,
	},
}

impl CooldownScope {
	/// 创建全局冷却作用域。
	pub const fn global() -> Self {
		Self::Global
	}

	/// 创建机器人冷却作用域。
	pub fn bot(bot_id: impl Into<SmolStr>) -> Self {
		Self::Bot { bot_id: bot_id.into() }
	}

	/// 创建好友冷却作用域。
	pub fn friend(bot_id: impl Into<SmolStr>, user_id: impl Into<SmolStr>) -> Self {
		Self::Friend { bot_id: bot_id.into(), user_id: user_id.into() }
	}

	/// 创建群组冷却作用域。
	pub fn group(bot_id: impl Into<SmolStr>, group_id: impl Into<SmolStr>) -> Self {
		Self::Group { bot_id: bot_id.into(), group_id: group_id.into() }
	}

	/// 创建群成员冷却作用域。
	pub fn group_member(
		bot_id: impl Into<SmolStr>,
		group_id: impl Into<SmolStr>,
		user_id: impl Into<SmolStr>,
	) -> Self {
		Self::GroupMember {
			bot_id: bot_id.into(),
			group_id: group_id.into(),
			user_id: user_id.into(),
		}
	}
}

impl Display for CooldownScope {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Global => f.write_str("global"),
			Self::Bot { bot_id } => write!(f, "bot:{bot_id}"),
			Self::Friend { bot_id, user_id } => write!(f, "bot:{bot_id}:userId:{user_id}"),
			Self::Group { bot_id, group_id } => {
				write!(f, "bot:{bot_id}:groupId:{group_id}")
			}
			Self::GroupMember { bot_id, group_id, user_id } => {
				write!(f, "bot:{bot_id}:groupId:{group_id}:userId:{user_id}")
			}
		}
	}
}

/// 一次消息冷却判定的结果。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CooldownState {
	/// 当前作用域未处于冷却中。
	Ready,

	/// 当前作用域仍在冷却中。
	CoolingDown {
		/// 当前窗口的剩余时间。
		remaining: Duration,
	},
}

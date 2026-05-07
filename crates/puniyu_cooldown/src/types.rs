use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 冷却范围枚举
///
/// 定义冷却的作用域，支持全局、机器人、好友、群组和群成员级别。
///
/// # 变体
///
/// - `Global` - 全局冷却，所有用户共享
/// - `Bot` - 机器人级别冷却，特定机器人的所有用户共享
/// - `Friend` - 好友级别冷却，特定机器人的特定好友
/// - `Group` - 群组级别冷却，特定机器人的特定群组
/// - `GroupMember` - 群成员级别冷却，特定群组中的特定成员
///
/// # 示例
///
/// ```rust
/// use puniyu_cooldown::CooldownScope;
///
/// // 全局冷却
/// let global = CooldownScope::Global;
///
/// // 机器人级别冷却
/// let bot = CooldownScope::Bot { bot_id: "123456" };
///
/// // 好友级别冷却
/// let friend = CooldownScope::Friend {
///     bot_id: "123456",
///     user_id: "789012",
/// };
///
/// // 群组级别冷却
/// let group = CooldownScope::Group {
///     bot_id: "123456",
///     group_id: "456789",
/// };
///
/// // 群成员级别冷却
/// let member = CooldownScope::GroupMember {
///     bot_id: "123456",
///     group_id: "456789",
///     user_id: "789012",
/// };
/// ```
#[derive(
	Debug,
	Default,
	Clone,
	PartialEq,
	Eq,
	Hash,
	EnumString,
	IntoStaticStr,
	Display,
	Deserialize,
	Serialize,
)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum CooldownScope<'c> {
	/// 全局冷却
	///
	/// 所有用户共享的冷却时间，适用于全局限制的功能。
	#[default]
	#[strum(serialize = "global")]
	Global,

	/// 机器人级别冷却
	///
	/// 特定机器人的所有用户共享的冷却时间。
	#[strum(serialize = "bot:{bot_id}")]
	Bot {
		/// 机器人 ID
		bot_id: &'c str,
	},

	/// 好友级别冷却
	///
	/// 特定机器人的特定好友的冷却时间。
	#[strum(serialize = "bot:{bot_id}:userId:{user_id}")]
	Friend {
		/// 机器人 ID
		bot_id: &'c str,
		/// 用户 ID
		user_id: &'c str,
	},

	/// 群组级别冷却
	///
	/// 特定机器人的特定群组的冷却时间，群内所有成员共享。
	#[strum(serialize = "bot:{bot_id}:groupId:{group_id}")]
	Group {
		/// 机器人 ID
		bot_id: &'c str,
		/// 群组 ID
		group_id: &'c str,
	},

	/// 群成员级别冷却
	///
	/// 特定群组中特定成员的冷却时间，最细粒度的冷却控制。
	#[strum(serialize = "bot:{bot_id}:groupId:{group_id}:userId:{user_id}")]
	GroupMember {
		/// 机器人 ID
		bot_id: &'c str,
		/// 群组 ID
		group_id: &'c str,
		/// 用户 ID
		user_id: &'c str,
	},
}

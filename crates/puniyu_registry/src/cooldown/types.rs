#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CooldownScope<'c> {
	/// 全局冷却
	Global,
	/// 机器人级别冷却
	Bot { bot_id: &'c str },
	/// 好友级别冷却
	Friend { bot_id: &'c str, user_id: &'c str },
	/// 群组级别冷却
	Group { bot_id: &'c str, group_id: &'c str },
	/// 群成员级别冷却
	GroupMember { bot_id: &'c str, group_id: &'c str, user_id: &'c str },
}

impl CooldownScope<'_> {
	pub(crate) fn make_key(&self) -> String {
		match self {
			Self::Global => "global".to_string(),
			Self::Bot { bot_id } => format!("bot:{}", bot_id),
			Self::Friend { bot_id, user_id } => format!("bot:{}:friend:{}", bot_id, user_id),
			Self::Group { bot_id, group_id } => format!("bot:{}:group:{}", bot_id, group_id),
			Self::GroupMember { bot_id, group_id, user_id } => {
				format!("bot:{}:group:{}:user:{}", bot_id, group_id, user_id)
			}
		}
	}
}

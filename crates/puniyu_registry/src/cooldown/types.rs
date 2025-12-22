#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CooldownScope<'a> {
	Friend { bot_id: &'a str, user_id: &'a str },
	Group { bot_id: &'a str, group_id: &'a str },
	GroupUser { bot_id: &'a str, group_id: &'a str, user_id: &'a str },
}

impl CooldownScope<'_> {
	pub fn make_key(&self) -> String {
		match self {
			Self::Friend { bot_id, user_id } => format!("bot:{}:friend:{}", bot_id, user_id),
			Self::Group { bot_id, group_id } => format!("bot:{}:group:{}", bot_id, group_id),
			Self::GroupUser { bot_id, group_id, user_id } => {
				format!("bot:{}:group:{}:user:{}", bot_id, group_id, user_id)
			}
		}
	}
}

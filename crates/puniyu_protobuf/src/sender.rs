use puniyu_sender::Sender;

include!(concat!(env!("OUT_DIR"), "/puniyu.sender.rs"));

impl_enum_from!(Sex => puniyu_sender::Sex {
	Male,
	Female,
	Unknown,
});

impl_enum_from!(Role => puniyu_sender::Role {
	Owner,
	Admin,
	Member,
	Unknown,
});

impl From<FriendSender> for puniyu_sender::FriendSender<'_> {
	fn from(value: FriendSender) -> Self {
		let sex = Sex::try_from(value.sex).unwrap_or_default();
		Self::new(value.user_id, value.nick, sex.into(), value.age)
	}
}

impl From<puniyu_sender::FriendSender<'_>> for FriendSender {
	fn from(value: puniyu_sender::FriendSender) -> Self {
		let sex = Sex::from(*value.sex());
		Self {
			user_id: value.user_id().to_string(),
			nick: value.name().map(|s| s.to_string()),
			sex: sex.into(),
			age: value.age(),
		}
	}
}

impl From<GroupSender> for puniyu_sender::GroupSender<'_> {
	fn from(value: GroupSender) -> Self {
		let sex = Sex::try_from(value.sex).unwrap_or_default();
		let role = Role::try_from(value.role).unwrap_or_default();
		Self::new(
			value.user_id,
			value.nick,
			sex.into(),
			value.age,
			role.into(),
			value.card,
			value.level,
			value.title,
		)
	}
}

impl From<puniyu_sender::GroupSender<'_>> for GroupSender {
	fn from(value: puniyu_sender::GroupSender) -> Self {
		let sex = Sex::from(*value.sex());
		let role = Role::from(*value.role());
		Self {
			user_id: value.user_id().to_string(),
			nick: value.name().map(|s| s.to_string()),
			sex: sex.into(),
			age: value.age(),
			role: role.into(),
			card: value.card().map(|s| s.to_string()),
			level: value.level(),
			title: value.title().map(|s| s.to_string()),
		}
	}
}

impl From<GroupTempSender> for puniyu_sender::GroupTempSender<'_> {
	fn from(value: GroupTempSender) -> Self {
		let sex = Sex::try_from(value.sex).unwrap_or_default();
		let role = Role::try_from(value.role).unwrap_or_default();
		Self::new(
			value.user_id,
			value.nick,
			sex.into(),
			value.age,
			role.into(),
			value.card,
			value.level,
			value.title,
		)
	}
}

impl From<puniyu_sender::GroupTempSender<'_>> for GroupTempSender {
	fn from(value: puniyu_sender::GroupTempSender) -> Self {
		let sex = Sex::from(*value.sex());
		let role = Role::from(*value.role());
		Self {
			user_id: value.user_id().to_string(),
			nick: value.name().map(|s| s.to_string()),
			sex: sex.into(),
			age: value.age(),
			role: role.into(),
			card: value.card().map(|s| s.to_string()),
			level: value.level(),
			title: value.title().map(|s| s.to_string()),
		}
	}
}

impl From<GuildSender> for puniyu_sender::GuildSender<'_> {
	fn from(value: GuildSender) -> Self {
		let sex = Sex::try_from(value.sex).unwrap_or_default();
		let role = Role::try_from(value.role).unwrap_or_default();
		Self::new(
			value.user_id,
			value.nick,
			sex.into(),
			value.age,
			role.into(),
			value.card,
			value.level,
			value.title,
		)
	}
}

impl From<puniyu_sender::GuildSender<'_>> for GuildSender {
	fn from(value: puniyu_sender::GuildSender) -> Self {
		let sex = Sex::from(*value.sex());
		let role = Role::from(*value.role());
		Self {
			user_id: value.user_id().to_string(),
			nick: value.name().map(|s| s.to_string()),
			sex: sex.into(),
			age: value.age(),
			role: role.into(),
			card: value.card().map(|s| s.to_string()),
			level: value.level(),
			title: value.title().map(|s| s.to_string()),
		}
	}
}

impl From<puniyu_sender::SenderType<'_>> for sender_type::SenderType {
	fn from(value: puniyu_sender::SenderType) -> Self {
		match value {
			puniyu_sender::SenderType::Friend(friend) => {
				sender_type::SenderType::FriendSender(friend.into())
			}
			puniyu_sender::SenderType::Group(group) => {
				sender_type::SenderType::GroupSender(group.into())
			}
			puniyu_sender::SenderType::GroupTemp(group_temp) => {
				sender_type::SenderType::GroupTempSender(group_temp.into())
			}
			puniyu_sender::SenderType::Guild(guild) => {
				sender_type::SenderType::GuildSender(guild.into())
			}
		}
	}
}

impl From<sender_type::SenderType> for puniyu_sender::SenderType<'_> {
	fn from(value: sender_type::SenderType) -> Self {
		match value {
			sender_type::SenderType::FriendSender(friend) => {
				puniyu_sender::SenderType::Friend(friend.into())
			}
			sender_type::SenderType::GroupSender(group) => {
				puniyu_sender::SenderType::Group(group.into())
			}
			sender_type::SenderType::GroupTempSender(group_temp) => {
				puniyu_sender::SenderType::GroupTemp(group_temp.into())
			}
			sender_type::SenderType::GuildSender(guild) => {
				puniyu_sender::SenderType::Guild(guild.into())
			}
		}
	}
}

impl From<FriendSender> for sender_type::SenderType {
	fn from(sender: FriendSender) -> Self {
		Self::FriendSender(sender)
	}
}


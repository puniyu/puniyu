use puniyu_types::sender;
include!(concat!(env!("OUT_DIR"), "/puniyu.sender.rs"));

impl From<Sex> for sender::Sex {
	fn from(sex: Sex) -> Self {
		match sex {
			Sex::Male => sender::Sex::Male,
			Sex::Female => sender::Sex::Female,
			Sex::Unknown => sender::Sex::Unknown,
		}
	}
}

impl From<sender::Sex> for Sex {
	fn from(value: sender::Sex) -> Self {
		match value {
			sender::Sex::Male => Sex::Male,
			sender::Sex::Female => Sex::Female,
			sender::Sex::Unknown => Sex::Unknown,
		}
	}
}

impl From<Role> for sender::Role {
	fn from(role: Role) -> Self {
		match role {
			Role::Owner => sender::Role::Owner,
			Role::Admin => sender::Role::Admin,
			Role::Member => sender::Role::Member,
			Role::Unknown => sender::Role::Unknown,
		}
	}
}

impl From<sender::Role> for Role {
	fn from(role: sender::Role) -> Self {
		match role {
			sender::Role::Owner => Role::Owner,
			sender::Role::Admin => Role::Admin,
			sender::Role::Member => Role::Member,
			sender::Role::Unknown => Role::Unknown,
		}
	}
}

impl From<FriendSender> for sender::FriendSender {
	fn from(value: FriendSender) -> Self {
		let sex = Sex::try_from(value.sex).unwrap();
		Self { user_id: value.user_id, nick: value.nick, sex: sex.into(), age: value.age.map(|age| age as u8) }
	}
}

impl From<sender::FriendSender> for FriendSender {
	fn from(value: sender::FriendSender) -> Self {
		let sex = Sex::from(value.sex);
		Self { user_id: value.user_id, nick: value.nick, sex: sex.into(), age: value.age.map(|age| age as u32) }
	}
}

impl From<GroupSender> for sender::GroupSender {
	fn from(value: GroupSender) -> Self {
		let sex = Sex::try_from(value.sex).unwrap();
		let role = Role::try_from(value.role).unwrap();
		Self {
			user_id: value.user_id,
			nick: value.nick,
			sex: sex.into(),
			age: value.age.map(|age| age as u8),
			role: role.into(),
			card: value.card,
			level: value.level.map(|level| level as u8),
			title: value.title,
		}
	}
}

impl From<sender::GroupSender> for GroupSender {
	fn from(value: sender::GroupSender) -> Self {
		let sex = Sex::from(value.sex);
		let role = Role::from(value.role);
		Self {
			user_id: value.user_id,
			nick: value.nick,
			sex: sex.into(),
			age: value.age.map(|age| age as u32),
			role: role.into(),
			card: value.card,
			level: value.level.map(|level| level as u32),
			title: value.title,
		}
	}
}

impl From<sender::SenderType> for sender_type::SenderType {
	fn from(value: sender::SenderType) -> Self {
		match value {
			sender::SenderType::Friend(friend) => {
				sender_type::SenderType::FriendSender(friend.into())
			}
			sender::SenderType::Group(group) => sender_type::SenderType::GroupSender(group.into()),
		}
	}
}

impl From<sender_type::SenderType> for sender::SenderType {
	fn from(value: sender_type::SenderType) -> Self {
		match value {
			sender_type::SenderType::FriendSender(friend) => {
				sender::SenderType::Friend(friend.into())
			}
			sender_type::SenderType::GroupSender(group) => sender::SenderType::Group(group.into()),
		}
	}
}

impl From<sender::SenderType> for SenderType {
	fn from(value: sender::SenderType) -> Self {
		Self { sender_type: Some(value.into()) }
	}
}

impl From<SenderType> for sender::SenderType {
	fn from(value: SenderType) -> Self {
		value.sender_type.unwrap().into()
	}
}

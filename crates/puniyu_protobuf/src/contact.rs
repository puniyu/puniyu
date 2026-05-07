use puniyu_contact::Contact as _;

include!(concat!(env!("OUT_DIR"), "/puniyu.contact.rs"));

impl_enum_from!(SceneType => puniyu_contact::SceneType {
	Friend,
	Group,
	GroupTemp,
	Guild,
});

impl From<puniyu_contact::FriendContact<'_>> for FriendContact {
	fn from(value: puniyu_contact::FriendContact) -> Self {
		Self { peer: value.peer().to_string(), name: value.name().map(|s| s.to_string()) }
	}
}

impl From<FriendContact> for puniyu_contact::FriendContact<'_> {
	fn from(value: FriendContact) -> Self {
		Self::new(value.peer, value.name)
	}
}
impl From<puniyu_contact::GroupContact<'_>> for GroupContact {
	fn from(value: puniyu_contact::GroupContact) -> Self {
		Self { peer: value.peer().to_string(), name: value.name().map(|s| s.to_string()) }
	}
}

impl From<GroupContact> for puniyu_contact::GroupContact<'_> {
	fn from(value: GroupContact) -> Self {
		Self::new(value.peer, value.name)
	}
}

impl From<puniyu_contact::GroupTempContact<'_>> for GroupTempContact {
	fn from(value: puniyu_contact::GroupTempContact) -> Self {
		Self { peer: value.peer().to_string(), name: value.name().map(|s| s.to_string()) }
	}
}

impl From<GroupTempContact> for puniyu_contact::GroupTempContact<'_> {
	fn from(value: GroupTempContact) -> Self {
		Self::new(value.peer, value.name)
	}
}

impl From<puniyu_contact::GuildContact<'_>> for GuildContact {
	fn from(value: puniyu_contact::GuildContact) -> Self {
		Self {
			peer: value.peer().to_string(),
			name: value.name().map(|s| s.to_string()),
			sub_name: value.sub_name().map(|s| s.to_string()),
		}
	}
}

impl From<GuildContact> for puniyu_contact::GuildContact<'_> {
	fn from(value: GuildContact) -> Self {
		Self::new(value.peer, value.name, value.sub_name)
	}
}

impl From<Contact> for puniyu_contact::ContactType<'_> {
	fn from(value: Contact) -> Self {
		match value.contact_type {
			Some(contact::ContactType::FriendContact(friend)) => {
				puniyu_contact::ContactType::Friend(friend.into())
			}
			Some(contact::ContactType::GroupContact(group)) => {
				puniyu_contact::ContactType::Group(group.into())
			}
			Some(contact::ContactType::GroupTempContact(group_temp)) => {
				puniyu_contact::ContactType::GroupTemp(group_temp.into())
			}
			Some(contact::ContactType::GuildContact(guild)) => {
				puniyu_contact::ContactType::Guild(guild.into())
			}
			None => panic!("puniyu_protobuf::Contact.contact_type cannot be None"),
		}
	}
}

impl From<puniyu_contact::ContactType<'_>> for Contact {
	fn from(value: puniyu_contact::ContactType<'_>) -> Self {
		match value {
			puniyu_contact::ContactType::Friend(friend) => {
				Contact { contact_type: Some(contact::ContactType::FriendContact(friend.into())) }
			}
			puniyu_contact::ContactType::Group(group) => {
				Contact { contact_type: Some(contact::ContactType::GroupContact(group.into())) }
			}
			puniyu_contact::ContactType::GroupTemp(group_temp) => Contact {
				contact_type: Some(contact::ContactType::GroupTempContact(group_temp.into())),
			},
			puniyu_contact::ContactType::Guild(guild) => {
				Contact { contact_type: Some(contact::ContactType::GuildContact(guild.into())) }
			}
		}
	}
}

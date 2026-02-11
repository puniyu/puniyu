use puniyu_types::contact::SceneType;
use puniyu_types::contact::{ContactType, FriendContact, GroupContact};

include!(concat!(env!("OUT_DIR"), "/puniyu.contact.rs"));

impl From<SceneType> for SceneType {
	fn from(scene_type: SceneType) -> Self {
		match scene_type {
			SceneType::Friend => SceneType::Friend,
			SceneType::Group => SceneType::Group,
		}
	}
}

impl From<SceneType> for SceneType {
	fn from(scene: SceneType) -> Self {
		match scene {
			SceneType::Friend => SceneType::Friend,
			SceneType::Group => SceneType::Group,
		}
	}
}

impl From<FriendContact> for Contact {
	fn from(value: FriendContact) -> Self {
		let scene = SceneType::from(value.scene);
		Self { scene: scene.into(), peer: value.peer, name: value.name }
	}
}

impl From<Contact> for FriendContact {
	fn from(value: Contact) -> Self {
		Self { scene: SceneType::Friend, peer: value.peer, name: value.name }
	}
}
impl From<GroupContact> for Contact {
	fn from(value: GroupContact) -> Self {
		let scene = SceneType::from(value.scene);
		Self { scene: scene.into(), peer: value.peer, name: value.name }
	}
}

impl From<Contact> for GroupContact {
	fn from(value: Contact) -> Self {
		Self { scene: SceneType::Group, peer: value.peer, name: value.name }
	}
}

impl From<Contact> for ContactType {
	fn from(value: Contact) -> Self {
		let peer = value.peer;
		let name = value.name;
		let scene = SceneType::try_from(value.scene).unwrap_or_default();
		match scene {
			SceneType::Friend => {
				ContactType::Friend(FriendContact { scene: SceneType::Friend, peer, name })
			}
			SceneType::Group => {
				ContactType::Group(GroupContact { scene: SceneType::Group, peer, name })
			}
		}
	}
}

impl From<ContactType> for Contact {
	fn from(value: ContactType) -> Self {
		match value {
			ContactType::Friend(friend) => {
				Contact { scene: SceneType::Friend.into(), peer: friend.peer, name: friend.name }
			}
			ContactType::Group(group) => {
				Contact { scene: SceneType::Group.into(), peer: group.peer, name: group.name }
			}
		}
	}
}

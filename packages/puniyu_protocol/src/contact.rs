use puniyu_types::contact::Scene;
use puniyu_types::contact::{ContactType, FriendContact, GroupContact};

include!(concat!(env!("OUT_DIR"), "/puniyu.contact.rs"));

impl From<SceneType> for Scene {
	fn from(scene_type: SceneType) -> Self {
		match scene_type {
			SceneType::Friend => Scene::Friend,
			SceneType::Group => Scene::Group,
		}
	}
}

impl From<Scene> for SceneType {
	fn from(scene: Scene) -> Self {
		match scene {
			Scene::Friend => SceneType::Friend,
			Scene::Group => SceneType::Group,
		}
	}
}

impl From<Contact> for ContactType {
	fn from(value: Contact) -> Self {
		let peer = value.peer;
		let name = value.name;
		let scene = SceneType::try_from(value.scene).unwrap();
		match scene {
			SceneType::Friend => {
				ContactType::Friend(FriendContact { scene: Scene::Friend, peer, name })
			}
			SceneType::Group => {
				ContactType::Group(GroupContact { scene: Scene::Group, peer, name })
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

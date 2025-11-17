use crate::Contact;

use super::Scene;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FriendContact {
	/// 事件来源
	pub scene: Scene,
	/// 好友ID
	pub peer: String,
	/// 好友名称
	pub name: Option<String>,
}


impl Contact for FriendContact {
	fn scene(&self) -> Scene {
		Scene::Friend
	}

	fn peer(&self) -> &str {
		&self.peer
	}

	fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}

	fn is_friend(&self) -> bool {
		true
	}

	fn is_group(&self) -> bool {
		false
	}
}


/// 构建好友事件
///
/// ## 参数
/// `peer`: 好友id
/// `name`: 好友昵称
#[macro_export]
macro_rules! contact_friend {
	($peer:expr, $name:expr) => {
		FriendContact { scene: Scene::Friend, peer: $peer.to_string(), name: Some($name.to_string()) }
	};
	(peer: $peer:expr, name: $name:expr) => {
		FriendContact { scene: Scene::Friend, peer: $peer.to_string(), name: Some($name.to_string()) }
	};
	($peer:expr) => {
		FriendContact { scene: Scene::Friend, peer: $peer.to_string(), name: None }
	};
	(peer: $peer:expr) => {
		FriendContact { scene: Scene::Friend, peer: $peer.to_string(), name: None }
	};
}

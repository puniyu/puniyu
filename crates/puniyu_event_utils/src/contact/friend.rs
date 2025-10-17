use super::Scene;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FriendContact {
	/// 事件来源
	pub scene: Scene,
	/// 好友ID
	pub peer: String,
	/// 好友名称
	pub name: String,
}

/// 构建好友事件
///
/// ## 参数
/// `peer`: 好友id
/// `name`: 好友昵称
#[macro_export]
macro_rules! contact_friend {
	($peer:expr, $name:expr) => {
		Contact::Friend(FriendContact {
			scene: Scene::Friend,
			peer: $peer.to_string(),
			name: $name.to_string(),
		})
	};
	(peer: $peer:expr, name: $name:expr) => {
		Contact::Friend(FriendContact {
			scene: Scene::Friend,
			peer: $peer.to_string(),
			name: $name.to_string(),
		})
	};
}

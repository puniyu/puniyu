use super::{Contact, Scene};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GroupContact {
	/// 事件来源
	pub scene: Scene,
	/// 群聊id
	pub peer: String,
	/// 群名称
	pub name: Option<String>,
}

impl Contact for GroupContact {
	fn scene(&self) -> Scene {
		Scene::Group
	}

	fn peer(&self) -> &str {
		self.peer.as_str()
	}

	fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}

	fn is_friend(&self) -> bool {
		false
	}

	fn is_group(&self) -> bool {
		true
	}
}


/// 构建群聊事件
///
/// ## 参数
/// `peer`: 群聊id
/// `name`: 群聊昵称（可选）
/// 
/// ## 示例
/// ```rust
/// use puniyu_types::contact_group;
/// use puniyu_types::contact::{Scene, GroupContact};
///
/// let contact = contact_group!("123456");
/// ```
#[macro_export]
macro_rules! contact_group {
	($peer:expr) => {
		GroupContact { scene: Scene::Group, peer: $peer.to_string(), name: None }
	};
	($peer:expr, $name:expr) => {
		GroupContact { scene: Scene::Group, peer: $peer.to_string(), name: Some($name.to_string()) }
	};
	(peer: $peer:expr) => {
		GroupContact { scene: Scene::Group, peer: $peer.to_string(), name: None }
	};
	(peer: $peer:expr, name: $name:expr) => {
		GroupContact { scene: Scene::Group, peer: $peer.to_string(), name: Some($name.to_string()) }
	};
}

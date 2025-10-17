use super::Scene;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GroupContact {
	/// 事件来源
	pub scene: Scene,
	/// 群聊id
	pub peer: String,
	/// 群名称
	pub name: String,
}

/// 构建群聊事件
///
/// ## 参数
/// `peer`: 群聊id
/// `name`: 群聊昵称
#[macro_export]
macro_rules! contact_group {
	($peer:expr, $name:expr) => {
		GroupContact { scene: Scene::Group, peer: $peer.to_string(), name: $name.to_string() }
	};
	(peer: $peer:expr, name: $name:expr) => {
		GroupContact { scene: Scene::Group, peer: $peer.to_string(), name: $name.to_string() }
	};
}

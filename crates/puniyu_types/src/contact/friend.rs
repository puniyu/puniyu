use super::{Contact, Scene};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
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
///
/// ## 示例
/// ```rust
/// use puniyu_types::contact_friend;
/// contact_friend!("2233");
/// ```
#[cfg(feature = "contact")]
#[macro_export]
macro_rules! contact_friend {
    ( $( $key:ident : $value:expr ),* $(,)? ) => {{
        let mut contact = $crate::contact::FriendContact {
            scene: $crate::contact::Scene::Friend,
            ..$crate::contact::FriendContact::default()
        };
        $(
            contact.$key = contact_friend!(@convert $key, $value);
        )*
        contact
    }};

    ($peer:expr, $name:expr) => {{
        let mut contact = $crate::contact::FriendContact {
            scene: $crate::contact::Scene::Friend,
            ..$crate::contact::FriendContact::default()
        };
        contact.peer = $peer.to_string();
        contact.name = Some($name.to_string());
        contact
    }};

    ($peer:expr) => {{
        let mut contact = $crate::contact::FriendContact {
            scene: $crate::contact::Scene::Friend,
            ..$crate::contact::FriendContact::default()
        };
        contact.peer = $peer.to_string();
        contact
    }};

    (@convert peer, $v:expr) => { $v.to_string() };
    (@convert name, None) => { None };
    (@convert name, $v:expr) => { Some($v.to_string()) };
}

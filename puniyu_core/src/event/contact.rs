use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, EnumString, Display, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum Scene {
	Group,
	Friend,
}

pub trait ContactBase {
	fn scene(&self) -> &str;
	fn peer(&self) -> &str;
	fn name(&self) -> &str;
}

pub struct FriendContact {
	/// 联系人 ID
	peer: String,
	/// 联系人名称
	name: Option<String>,
}

impl FriendContact {
	pub fn new(peer: String, name: Option<String>) -> Self {
		Self { peer, name }
	}
}
impl ContactBase for FriendContact {
	fn scene(&self) -> &str {
		Scene::Friend.into()
	}

	fn peer(&self) -> &str {
		&self.peer
	}

	fn name(&self) -> &str {
		self.name.as_deref().unwrap_or("")
	}
}

pub enum Contact {
	Friend(FriendContact),
}

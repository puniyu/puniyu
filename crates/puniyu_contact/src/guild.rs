use bon::Builder;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use puniyu_core::contact::Contact;

use crate::SceneType;

/// 频道联系人
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Builder)]
#[builder(on(SmolStr, into))]
pub struct GuildContact {
	/// 频道id
	peer: SmolStr,
	/// 频道名称
	name: Option<SmolStr>,
	/// 子频道名称
	sub_name: Option<SmolStr>,
}

impl GuildContact {
	pub fn new<N>(peer: N, name: Option<N>, sub_name: Option<N>) -> Self
	where
		N: Into<SmolStr>,
	{
		Self { peer: peer.into(), name: name.map(Into::into), sub_name: sub_name.map(Into::into) }
	}
	/// 获取子频道名称
	pub fn sub_name(&self) -> Option<&str> {
		self.sub_name.as_deref()
	}
}

impl Contact for GuildContact {
	type Scene = SceneType;
	fn scene(&self) -> Self::Scene {
		SceneType::Guild
	}

	fn peer(&self) -> &str {
		self.peer.as_ref()
	}

	fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}
}

/// 构建频道联系人。
///
/// # 用法
///
/// ## 仅指定 peer
///
/// ```rust
/// use puniyu_contact::contact_guild;
///
/// let guild = contact_guild!("123");
/// ```
///
/// ## 指定 peer、name 和 sub_name
///
/// ```rust
/// use puniyu_contact::contact_guild;
///
/// let guild = contact_guild!("123", "Guild Channel", sub_name: "General");
/// ```
///
/// ## 使用命名字段
///
/// ```rust
/// use puniyu_contact::contact_guild;
///
/// let guild = contact_guild!(
///     peer: "123",
///     name: "Guild Channel",
///     sub_name: "General",
/// );
/// ```
#[macro_export]
macro_rules! contact_guild {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::GuildContact::builder()
            $(
                .$key($value)
            )*
            .build()
    }};
    ($peer:expr, $name:expr, sub_name: $sub_name:expr) => {{
        $crate::GuildContact::builder()
            .peer($peer)
            .name($name)
			.sub_name($sub_name)
            .build()
    }};
    ($peer:expr, $name:expr) => {{
        $crate::GuildContact::builder().peer($peer).name($name).build()
    }};

    ($peer:expr) => {{
        $crate::GuildContact::builder().peer($peer).build()
    }};
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Contact;

	#[test]
	fn test_new_basic() {
		let guild = GuildContact::new("123", Some("Guild Channel"), Some("General"));

		assert_eq!(guild.peer(), "123");
		assert_eq!(guild.name(), Some("Guild Channel"));
		assert_eq!(guild.sub_name(), Some("General"));
		assert_eq!(guild.scene(), SceneType::Guild);
	}

	#[test]
	fn test_new_none_values() {
		let guild = GuildContact::new("g1", None::<&str>, None::<&str>);

		assert_eq!(guild.peer(), "g1");
		assert_eq!(guild.name(), None);
		assert_eq!(guild.sub_name(), None);
		assert_eq!(guild.scene(), SceneType::Guild);
	}

	#[test]
	fn test_new_owned_string_creation() {
		let guild = GuildContact::new(
			String::from("123"),
			Some(String::from("Guild Channel")),
			Some(String::from("General")),
		);

		assert_eq!(guild.peer(), "123");
		assert_eq!(guild.name(), Some("Guild Channel"));
		assert_eq!(guild.sub_name(), Some("General"));
	}

	#[test]
	fn test_new_builder_equivalence() {
		let a = GuildContact::new("123", Some("Guild Channel"), Some("General"));
		let b =
			GuildContact::builder().peer("123").name("Guild Channel").sub_name("General").build();

		assert_eq!(a, b);
	}

	#[test]
	fn test_serde_roundtrip() {
		let guild = GuildContact::new("123", Some("Guild Channel"), Some("General"));

		let json = serde_json::to_string(&guild).expect("serialize");
		let restored: GuildContact = serde_json::from_str(&json).expect("deserialize");

		assert_eq!(guild, restored);
	}
}

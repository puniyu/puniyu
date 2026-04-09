use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{Contact, SceneType};

/// 频道联系人
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[serde(bound(deserialize = "'de: 'c"))]
#[builder(setter(into), pattern = "owned")]
pub struct GuildContact<'c> {
	/// 频道id
	#[serde(borrow)]
	pub peer: Cow<'c, str>,
	/// 频道名称
	#[builder(default, setter(into, strip_option))]
	#[serde(borrow)]
	pub name: Option<Cow<'c, str>>,
	/// 子频道名称
	#[builder(default, setter(into, strip_option))]
	#[serde(borrow)]
	pub sub_name: Option<Cow<'c, str>>,
}

impl<'c> GuildContact<'c> {
	pub fn new<N>(peer: N, name: N, sub_name: N) -> Self
	where
		N: Into<Cow<'c, str>>,
	{
		Self { peer: peer.into(), name: Some(name.into()), sub_name: Some(sub_name.into()) }
	}
	pub fn builder() -> GuildContactBuilder<'c> {
		GuildContactBuilder::default()
	}
	/// 获取子频道名称
	pub fn sub_name(&self) -> Option<&str> {
		self.sub_name.as_deref()
	}
}

impl<'c> Contact for GuildContact<'c> {
	fn scene(&self) -> &SceneType {
		&SceneType::Guild
	}

	fn peer(&self) -> &str {
		self.peer.as_ref()
	}

	fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}
}

#[macro_export]
macro_rules! contact_guild {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::GuildContactBuilder::default()
		$(
			.$key($value)
		)*
		.build()
		.expect("Failed to build GuildContact")
    }};

    ($peer:expr, $name:expr) => {{
        $crate::GuildContactBuilder::default()
            .peer($peer)
            .name($name)
            .build()
            .expect("Failed to build GuildContact")
    }};

    ($peer:expr) => {{
        $crate::GuildContactBuilder::default()
            .peer($peer)
            .build()
            .expect("Failed to build GuildContact")
    }};
}

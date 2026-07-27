use std::borrow::Cow;

/// 行为标识符。
pub enum ActionId<'c> {
	/// 通过索引标识。
	Id(u64),
	/// 通过名称标识。
	Name(Cow<'c, str>),
}

impl From<u64> for ActionId<'_> {
	fn from(id: u64) -> Self {
		Self::Id(id)
	}
}

impl<'c> From<&'c str> for ActionId<'c> {
	fn from(name: &'c str) -> Self {
		Self::Name(Cow::Borrowed(name))
	}
}

impl From<String> for ActionId<'_> {
	fn from(name: String) -> Self {
		Self::Name(Cow::Owned(name))
	}
}

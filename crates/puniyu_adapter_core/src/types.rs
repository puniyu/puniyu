use std::borrow::Cow;

/// 适配器标识符。
pub enum AdapterId<'a> {
	/// 通过索引标识。
	Index(u64),
	/// 通过名称标识。
	Name(Cow<'a, str>),
}

impl From<u64> for AdapterId<'_> {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl<'a> From<&'a str> for AdapterId<'a> {
	fn from(name: &'a str) -> Self {
		Self::Name(Cow::Borrowed(name))
	}
}

impl From<String> for AdapterId<'_> {
	fn from(name: String) -> Self {
		Self::Name(Cow::Owned(name))
	}
}

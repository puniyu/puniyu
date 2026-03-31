use std::borrow::Cow;

/// 加载器标识符。
#[derive(Debug, Clone, PartialEq)]
pub enum LoaderId<'l> {
	/// 数字索引标识符
	Index(u64),
	/// 名称字符串标识符
	Name(Cow<'l, str>),
}

impl From<u64> for LoaderId<'_> {
	fn from(id: u64) -> Self {
		Self::Index(id)
	}
}

impl<'l> From<&'l str> for LoaderId<'l> {
	fn from(name: &'l str) -> Self {
		Self::Name(Cow::Borrowed(name))
	}
}

impl From<String> for LoaderId<'_> {
	fn from(name: String) -> Self {
		Self::Name(Cow::Owned(name))
	}
}

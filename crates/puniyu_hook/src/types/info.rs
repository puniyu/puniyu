use puniyu_common::source::SourceType;
use std::borrow::Cow;
use std::fmt::Debug;
use std::sync::Arc;

/// 钩子信息。
#[derive(Clone)]
pub struct HookInfo {
	/// 钩子来源。
	pub source: SourceType,
	/// 钩子实例。
	pub builder: Arc<dyn crate::Hook>,
}

impl Debug for HookInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("HookInfo").field(&self.source).finish()
	}
}

impl PartialEq for HookInfo {
	fn eq(&self, other: &Self) -> bool {
		self.source == other.source && self.builder.name() == other.builder.name()
	}
}

/// 钩子标识符。
pub enum HookId<'h> {
	/// 注册索引。
	Index(u64),
	/// 钩子名称。
	Name(Cow<'h, str>),
	/// 来源类型。
	Source(SourceType),
}

impl From<u64> for HookId<'_> {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl<'h> From<&'h str> for HookId<'h> {
	fn from(name: &'h str) -> Self {
		Self::Name(Cow::Borrowed(name))
	}
}

impl From<String> for HookId<'_> {
	fn from(name: String) -> Self {
		Self::Name(Cow::Owned(name))
	}
}

impl From<SourceType> for HookId<'_> {
	fn from(source: SourceType) -> Self {
		Self::Source(source)
	}
}

use std::borrow::Cow;

/// 任务 ID 枚举。
///
/// 用于按索引或名称查找/卸载任务。
#[derive(Debug, Clone)]
pub enum TaskId<'t> {
	/// 任务索引 ID
	Index(u64),
	/// 任务名称
	Name(Cow<'t, str>),
}

impl From<u64> for TaskId<'_> {
	#[inline]
	fn from(value: u64) -> Self {
		Self::Index(value)
	}
}

impl<'t> From<&'t str> for TaskId<'t> {
	#[inline]
	fn from(value: &'t str) -> Self {
		Self::Name(Cow::Borrowed(value))
	}
}

impl From<String> for TaskId<'_> {
	#[inline]
	fn from(value: String) -> Self {
		Self::Name(Cow::Owned(value))
	}
}

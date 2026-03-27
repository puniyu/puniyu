/// 适配器标识符。
pub enum AdapterId<'a> {
	/// 通过索引标识。
	Index(u64),
	/// 通过名称标识。
	Name(&'a str),
}

impl<'a> From<u64> for AdapterId<'a> {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl<'a> From<&'a str> for AdapterId<'a> {
	fn from(self_id: &'a str) -> Self {
		Self::Name(self_id)
	}
}

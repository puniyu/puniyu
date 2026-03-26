/// 适配器 ID
///
/// 用于标识适配器的枚举类型，支持通过索引或名称来标识。
///
/// # 示例
///
/// ```rust
/// use puniyu_adapter::types::AdapterId;
///
/// // 通过索引创建
/// let id1: AdapterId = 0u64.into();
///
/// // 通过名称创建
/// let id2: AdapterId = "my_adapter".into();
/// ```
pub enum AdapterId<'a> {
	/// 通过索引标识
	Index(u64),
	/// 通过名称标识
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

/// 加载器标识符
///
/// 用于在注册表中标识和查找加载器的枚举类型。
///
/// # 变体
///
/// - `Index` - 使用数字索引标识加载器
/// - `Name` - 使用名称字符串标识加载器
///
/// # 示例
///
/// ```rust
/// use puniyu_loader::LoaderId;
///
/// // 使用索引创建
/// let id1 = LoaderId::from(42u64);
///
/// // 使用名称创建
/// let id2 = LoaderId::from("my_loader");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum LoaderId<'l> {
	/// 数字索引标识符
	Index(u64),
	/// 名称字符串标识符
	Name(&'l str),
}

impl From<u64> for LoaderId<'_> {
	fn from(id: u64) -> Self {
		Self::Index(id)
	}
}

impl<'l> From<&'l str> for LoaderId<'l> {
	fn from(name: &'l str) -> Self {
		Self::Name(name)
	}
}

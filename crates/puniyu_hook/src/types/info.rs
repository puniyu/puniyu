use puniyu_common::source::SourceType;
use std::fmt::Debug;
use std::sync::Arc;

/// 钩子信息
///
/// 包含钩子的来源和构建器。
///
/// # 字段
///
/// - `source` - 钩子的来源类型
/// - `builder` - 钩子的构建器（实现了 `Hook` trait）
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_hook::types::HookInfo;
/// use puniyu_common::source::SourceType;
/// use std::sync::Arc;
///
/// let hook_info = HookInfo {
///     source: SourceType::Internal,
///     builder: Arc::new(my_hook),
/// };
/// ```
#[derive(Clone)]
pub struct HookInfo {
	/// 钩子来源
	pub source: SourceType,
	/// 钩子构建器
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

/// 钩子标识符
///
/// 用于在注册表中标识钩子的枚举类型。
///
/// # 变体
///
/// - `Index` - 使用注册表索引标识
/// - `Name` - 使用钩子名称标识
/// - `Source` - 使用来源类型标识
///
/// # 示例
///
/// ```rust
/// use puniyu_hook::types::HookId;
///
/// // 使用索引
/// let id1: HookId = 123u64.into();
///
/// // 使用名称
/// let id2: HookId = "my_hook".into();
/// ```
pub enum HookId<'h> {
	/// 注册表索引
	Index(u64),
	/// 钩子名称
	Name(&'h str),
	/// 来源类型
	Source(SourceType),
}

impl From<u64> for HookId<'_> {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl<'h> From<&'h str> for HookId<'h> {
	fn from(name: &'h str) -> Self {
		Self::Name(name)
	}
}

impl From<SourceType> for HookId<'_> {
	fn from(source: SourceType) -> Self {
		Self::Source(source)
	}
}

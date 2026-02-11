/// 机器人标识符
///
/// 用于在注册表中标识机器人的枚举类型。
///
/// # 变体
///
/// - `Index` - 使用注册表索引标识
/// - `SelfId` - 使用机器人自身 ID（UIN）标识
///
/// # 示例
///
/// ```rust
/// use puniyu_bot::types::BotId;
///
/// // 使用索引
/// let id1: BotId = 123u64.into();
///
/// // 使用 UIN
/// let id2: BotId = "123456".into();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BotId<'b> {
	/// 注册表索引
	Index(u64),
	/// 机器人 UIN
	SelfId(&'b str),
}

impl<'b> From<u64> for BotId<'b> {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl<'b> From<&'b str> for BotId<'b> {
	fn from(name: &'b str) -> Self {
		Self::SelfId(name)
	}
}

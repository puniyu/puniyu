/// 机器人标识符。
///
/// 用于在 [`BotRegistry`](crate::BotRegistry) 中按注册索引或机器人 UIN 定位实例。
///
/// # 示例
///
/// ```rust
/// use puniyu_bot::BotId;
///
/// let index: BotId = 123u64.into();
/// let self_id: BotId = "123456".into();
///
/// assert_eq!(index, BotId::Index(123));
/// assert_eq!(self_id, BotId::SelfId("123456"));
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

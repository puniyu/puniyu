mod sex;
#[doc(inline)]
pub use sex::Sex;
mod role;
#[doc(inline)]
pub use role::Role;

/// 发送者 trait
///
/// 定义发送者的通用接口，所有发送者类型都需要实现此 trait。
///
/// # 必需方法
///
/// - `user_id()` - 获取发送者 ID
/// - `name()` - 获取发送者昵称（可选）
/// - `sex()` - 获取发送者性别
/// - `age()` - 获取发送者年龄（可选）
///
/// # 示例
///
/// ```rust
/// use puniyu_sender::{Sender, FriendSender, Sex};
///
/// let sender = FriendSender {
///     user_id: "123456",
///     nick: Some("Alice"),
///     sex: Sex::Female,
///     age: Some(25),
/// };
///
/// // 使用 Sender trait 方法
/// assert_eq!(sender.user_id(), "123456");
/// assert_eq!(sender.name(), Some("Alice"));
/// assert_eq!(sender.sex(), &Sex::Female);
/// assert_eq!(sender.age(), Some(25));
/// ```
pub trait Sender: Send + Sync {
	/// 获取发送者 ID
	///
	/// # 返回值
	///
	/// 返回发送者的唯一标识符。
	fn user_id(&self) -> &str;

	/// 获取发送者昵称
	///
	/// # 返回值
	///
	/// 返回发送者的昵称，如果未设置则返回 `None`。
	fn name(&self) -> Option<&str>;

	/// 获取发送者性别
	///
	/// # 返回值
	///
	/// 返回发送者的性别信息。
	fn sex(&self) -> &Sex;

	/// 获取发送者年龄
	///
	/// # 返回值
	///
	/// 返回发送者的年龄，如果未设置则返回 `None`。
	fn age(&self) -> Option<u32>;
}

impl<T: Sender> Sender for &T {
	fn user_id(&self) -> &str {
		(**self).user_id()
	}

	fn name(&self) -> Option<&str> {
		(**self).name()
	}

	fn sex(&self) -> &Sex {
		(**self).sex()
	}

	fn age(&self) -> Option<u32> {
		(**self).age()
	}
}

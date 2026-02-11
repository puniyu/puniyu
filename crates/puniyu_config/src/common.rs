use serde_repr::{Deserialize_repr, Serialize_repr};

/// Bot 响应模式枚举
///
/// 定义 Bot 在不同场景下的响应行为。
///
/// # 示例
///
/// ```rust
/// use puniyu_config::ReactiveMode;
///
/// let mode = ReactiveMode::AtBot;
/// match mode {
///     ReactiveMode::All => println!("响应所有消息"),
///     ReactiveMode::AtBot => println!("仅响应 @ Bot 的消息"),
///     ReactiveMode::Alias => println!("仅响应使用别名的消息"),
///     ReactiveMode::AtOrAlias => println!("响应 @ Bot 或使用别名的消息"),
///     ReactiveMode::Master => println!("仅响应主人的消息"),
/// }
/// ```
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ReactiveMode {
	/// 响应所有消息
	///
	/// Bot 会响应所有接收到的消息，不做任何过滤
	All = 0,

	/// 仅响应 @ Bot 的消息
	///
	/// 只有当消息中包含 @ Bot 时才会响应
	AtBot = 1,

	/// 仅响应使用别名的消息
	///
	/// 只有当消息以配置的别名开头时才会响应
	Alias = 2,

	/// 响应 @ Bot 或使用别名的消息
	///
	/// 当消息包含 @ Bot 或以别名开头时都会响应
	AtOrAlias = 3,

	/// 仅响应主人的消息
	///
	/// 只响应配置中定义的主人发送的消息
	Master = 4,
}

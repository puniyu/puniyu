mod scene;
#[doc(inline)]
pub use scene::SceneType;

/// 联系人 Trait
///
/// 定义联系人的统一接口，所有联系人类型都必须实现此 trait。
///
/// # 方法
///
/// - `scene()` - 获取场景类型（好友或群聊）
/// - `peer()` - 获取联系人 ID
/// - `name()` - 获取联系人名称（可选）
/// - `is_friend()` - 判断是否为好友场景
/// - `is_group()` - 判断是否为群聊场景
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::{Contact, FriendContact, SceneType};
///
/// let friend = FriendContact {
///     scene: SceneType::Friend,
///     peer: "123456",
///     name: Some("Alice"),
/// };
///
/// // 使用 Contact trait 方法
/// assert_eq!(friend.peer(), "123456");
/// assert_eq!(friend.name(), Some("Alice"));
/// assert!(friend.is_friend());
/// assert!(!friend.is_group());
/// ```
///
/// # 泛型使用
///
/// 可以编写接受任何实现 `Contact` trait 的类型的泛型函数：
///
/// ```rust
/// use puniyu_contact::{Contact, FriendContact, GroupContact, SceneType};
///
/// fn print_contact_info<C: Contact>(contact: &C) {
///     println!("ID: {}", contact.peer());
///     if let Some(name) = contact.name() {
///         println!("Name: {}", name);
///     }
///     println!("Is friend: {}", contact.is_friend());
/// }
///
/// let friend = FriendContact {
///     scene: SceneType::Friend,
///     peer: "123456",
///     name: Some("Alice"),
/// };
/// print_contact_info(&friend);
///
/// let group = GroupContact {
///     scene: SceneType::Group,
///     peer: "789012",
///     name: Some("Dev Team"),
/// };
/// print_contact_info(&group);
/// ```
pub trait Contact: Send + Sync {
	/// 获取场景类型
	///
	/// 返回联系人所属的场景类型（好友或群聊）。
	fn scene(&self) -> &SceneType;

	/// 获取联系人 ID
	///
	/// 返回联系人的唯一标识符。
	fn peer(&self) -> &str;

	/// 获取联系人名称
	///
	/// 返回联系人的名称，如果未设置则返回 `None`。
	fn name(&self) -> Option<&str>;

	/// 判断是否为好友场景
	///
	/// # 返回值
	///
	/// 如果是好友场景返回 `true`，否则返回 `false`。
	fn is_friend(&self) -> bool {
		matches!(self.scene(), SceneType::Friend)
	}

	/// 判断是否为群聊场景
	///
	/// # 返回值
	///
	/// 如果是群聊场景返回 `true`，否则返回 `false`。
	fn is_group(&self) -> bool {
		matches!(self.scene(), SceneType::Group)
	}
}

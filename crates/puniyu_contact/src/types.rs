use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

pub trait Contact: Send + Sync {

	/// 获取场景类型
	/// 
	/// # 返回值
	/// 
	/// 返回场景类型 [`SceneType`]。
	fn scene(&self) -> SceneType;

	/// 获取联系人 ID
	///
	/// # 返回值
	///
	/// 返回联系人的唯一标识符 [`str`]。
	fn peer(&self) -> &str;

	/// 获取联系人名称
	///
	/// # 返回值
	///
	/// 返回联系人的名称 [`Option<&str>`],如果未设置则返回 [`None`]。
	fn name(&self) -> Option<&str>;
}

impl<T: Contact + ?Sized> Contact for &T {
	fn scene(&self) -> SceneType {
		(**self).scene()
	}
	fn peer(&self) -> &str {
		(**self).peer()
	}
	fn name(&self) -> Option<&str> {
		(**self).name()
	}
}

impl PartialEq for dyn Contact{
	fn eq(&self, other: &Self) -> bool {
		self.scene() == other.scene() && self.peer() == other.peer() && self.name() == other.name()
	}
}

impl Eq for dyn Contact {}


/// 场景类型
///
/// 定义联系人所属的场景类型，用于区分好友、群聊、群临时和频道消息。
///
/// # 变体
///
/// - `Friend` - 好友场景，表示一对一聊天
/// - `Group` - 群聊场景，表示群组聊天
/// - `GroupTemp` - 群临时场景
/// - `Guild` - 频道消息场景
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::SceneType;
/// use std::str::FromStr;
///
/// let scene = SceneType::Friend;
/// assert_eq!(scene.to_string(), "friend");
///
/// let scene = SceneType::from_str("group").unwrap();
/// assert_eq!(scene, SceneType::Group);
/// ```
///
/// # 序列化
///
/// 该枚举实现了 `Serialize` 和 `Deserialize`，可以直接用于 JSON 序列化：
///
/// ```rust
/// use puniyu_contact::SceneType;
/// use serde_json;
///
/// let scene = SceneType::Friend;
/// let json = serde_json::to_string(&scene).unwrap();
/// assert!(json.contains("friend"));
/// ```
#[derive(
	Debug,
	Default,
	Clone,
	Copy,
	PartialEq,
	Eq,
	Hash,
	EnumString,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SceneType {
	#[default]
	/// 群聊场景
	Group,
	/// 群临时场景
	GroupTemp,
	/// 好友场景
	Friend,
	/// 频道消息场景
	Guild,
}

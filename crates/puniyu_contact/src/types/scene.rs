use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 场景类型
///
/// 定义联系人所属的场景类型，用于区分好友和群聊。
///
/// # 变体
///
/// - `Friend` - 好友场景，表示一对一聊天
/// - `Group` - 群聊场景，表示群组聊天
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
	/// 好友场景
	Friend,
}

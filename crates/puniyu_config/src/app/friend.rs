use serde::{Deserialize, Serialize};

/// 应用级好友配置结构
///
/// 定义全局的好友访问控制策略，通过白名单和黑名单管理好友权限。
///
/// # 配置优先级
///
/// 黑名单优先级高于白名单。如果同一个好友同时出现在两个列表中，
/// 该好友会被禁止访问。
///
/// # 使用场景
///
/// - 白名单模式：只允许特定好友使用 Bot
/// - 黑名单模式：禁止特定好友使用 Bot
/// - 混合模式：在白名单基础上排除特定好友
///
/// # 示例
///
/// ```toml
/// [friend]
/// # 只允许这些好友使用
/// enable_list = ["user_123", "user_456"]
/// # 禁止这些好友使用（优先级更高）
/// disable_list = ["user_789"]
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FriendConfig {
	/// 好友白名单
	///
	/// 列出允许使用 Bot 的好友 ID
	#[serde(default)]
	enable_list: Vec<String>,

	/// 好友黑名单
	///
	/// 列出禁止使用 Bot 的好友 ID，优先级高于白名单
	#[serde(default)]
	disable_list: Vec<String>,
}

impl FriendConfig {
	/// 获取好友白名单
	pub fn enable_list(&self) -> &Vec<String> {
		&self.enable_list
	}

	/// 获取好友黑名单
	pub fn disable_list(&self) -> &Vec<String> {
		&self.disable_list
	}
}

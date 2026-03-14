use serde::{Deserialize, Serialize};

/// 应用级群组配置结构
///
/// 定义全局的群组访问控制策略，通过白名单和黑名单管理群组权限。
///
/// # 配置优先级
///
/// 黑名单优先级高于白名单。如果同一个群组同时出现在两个列表中，
/// 该群组会被禁止访问。
///
/// # 使用场景
///
/// - 白名单模式：只允许 Bot 在特定群组中工作
/// - 黑名单模式：禁止 Bot 在特定群组中工作
/// - 混合模式：在白名单基础上排除特定群组
///
/// # 示例
///
/// ```toml
/// [group]
/// # 只允许在这些群组中工作
/// enable_list = ["group_123", "group_456"]
/// # 禁止在这些群组中工作（优先级更高）
/// disable_list = ["group_789"]
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GroupConfig {
	/// 群组白名单
	///
	/// 列出允许 Bot 工作的群组 ID
	#[serde(default)]
	enable_list: Vec<String>,

	/// 群组黑名单
	///
	/// 列出禁止 Bot 工作的群组 ID，优先级高于白名单
	#[serde(default)]
	disable_list: Vec<String>,
}

impl GroupConfig {
	/// 获取群组白名单
	pub fn enable_list(&self) -> &Vec<String> {
		&self.enable_list
	}

	/// 获取群组黑名单
	pub fn disable_list(&self) -> &Vec<String> {
		&self.disable_list
	}
}

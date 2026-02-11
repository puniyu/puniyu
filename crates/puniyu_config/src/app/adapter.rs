use serde::{Deserialize, Serialize};

/// 适配器配置结构
///
/// 控制启用哪些消息适配器。
///
/// # 示例
///
/// ```toml
/// [adapter]
/// console = true
/// server = true
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
	/// 是否启用控制台适配器
	///
	/// 控制台适配器允许通过命令行与 Bot 交互
	#[serde(default = "default_adapter")]
	console: bool,

	/// 是否启用服务器适配器
	///
	/// 服务器适配器允许通过 HTTP 接口与 Bot 交互
	#[serde(default = "default_adapter")]
	server: bool,
}

impl AdapterConfig {
	/// 是否启用控制台适配器
	///
	/// # 返回值
	///
	/// 返回 `true` 表示启用控制台适配器
	pub fn console(&self) -> bool {
		self.console
	}

	/// 是否启用服务器适配器
	///
	/// # 返回值
	///
	/// 返回 `true` 表示启用服务器适配器
	pub fn server(&self) -> bool {
		self.server
	}
}

impl Default for AdapterConfig {
	#[inline]
	fn default() -> Self {
		Self { console: default_adapter(), server: default_adapter() }
	}
}
fn default_adapter() -> bool {
	true
}

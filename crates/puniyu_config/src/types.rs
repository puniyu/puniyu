use std::path::PathBuf;
use toml::Value;

/// 配置 trait
///
/// 定义自定义配置需要实现的接口。
///
/// # 使用场景
///
/// 当需要为插件或模块添加自定义配置时，实现此 trait 可以将配置
/// 集成到框架的配置管理系统中。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_config::types::Config;
/// use toml::Value;
///
/// struct MyPluginConfig {
///     api_key: String,
///     timeout: u64,
/// }
///
/// impl Config for MyPluginConfig {
///     fn name(&self) -> &'static str {
///         "my_plugin"
///     }
///
///     fn config(&self) -> Value {
///         toml::toml! {
///             api_key = self.api_key
///             timeout = self.timeout
///         }
///     }
/// }
/// ```
pub trait Config: Send + Sync + 'static {
	/// 配置文件名称
	///
	/// # 返回值
	///
	/// 返回配置文件的名称（不含扩展名），用于标识配置
	fn name(&self) -> &'static str;

	/// 配置项
	///
	/// # 返回值
	///
	/// 返回配置的 TOML 值表示
	fn config(&self) -> Value;
}

/// 配置信息结构
///
/// 存储配置文件的路径和内容。
///
/// # 字段
///
/// - `path`: 配置文件的完整路径
/// - `value`: 配置文件的 TOML 值表示
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigInfo {
	/// 配置文件路径
	pub path: PathBuf,
	/// 配置内容
	pub value: Value,
}

/// 配置标识符枚举
///
/// 用于通过索引或路径来标识配置。
///
/// # 变体
///
/// - `Index`: 通过数字索引标识配置
/// - `Path`: 通过文件路径标识配置
///
/// # 示例
///
/// ```rust
/// use puniyu_config::types::ConfigId;
/// use std::path::PathBuf;
///
/// // 通过索引创建
/// let id1: ConfigId = 0u64.into();
///
/// // 通过路径创建
/// let id2: ConfigId = PathBuf::from("config/app.toml").into();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigId {
	/// 通过索引标识
	Index(u64),
	/// 通过路径标识
	Path(PathBuf),
}

impl From<u64> for ConfigId {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl From<PathBuf> for ConfigId {
	fn from(path: PathBuf) -> Self {
		Self::Path(path)
	}
}

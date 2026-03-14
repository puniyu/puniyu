//! # puniyu_config_core
//!
//! 配置核心 trait 和注册表。
//!
//! ## 使用
//!
//! ```rust
//! use puniyu_config_core::Config;
//! use std::path::Path;
//! use toml::Value;
//!
//! // 实现 Config trait
//! struct MyConfig;
//!
//! impl Config for MyConfig {
//!     fn name(&self) -> &'static str {
//!         "my_config"
//!     }
//!
//!     fn path(&self) -> &'static Path {
//!         Path::new("config/my_config.toml")
//!     }
//!
//!     fn config(&self) -> Value {
//!         toml::toml! {
//!             enabled = true
//!             timeout = 30
//!         }
//!     }
//! }
//! ```

#[cfg(feature = "registry")]
mod registry;
#[doc(inline)]
#[cfg(feature = "registry")]
pub use registry::ConfigRegistry;
mod types;
#[doc(inline)]
pub use types::*;
use std::path::Path;
use toml::Value;

/// 配置 trait
///
/// 定义配置的基本接口，用于外部包实现自定义配置。
///
/// # 示例
///
/// ```rust
/// use puniyu_config_core::Config;
/// use std::path::Path;
/// use toml::Value;
///
/// struct AppConfig;
///
/// impl Config for AppConfig {
///     fn name(&self) -> &'static str {
///         "app"
///     }
///
///     fn path(&self) -> &'static Path {
///         Path::new("config/app.toml")
///     }
///
///     fn config(&self) -> Value {
///         toml::toml! {
///             version = "1.0.0"
///             debug = false
///         }
///     }
/// }
/// ```
pub trait Config: Send + Sync + 'static {
	/// 返回配置名称
	///
	/// 配置的唯一标识符，用于区分不同的配置。
	fn name(&self) -> &'static str;

	/// 返回配置文件路径
	///
	/// 配置文件在文件系统中的路径。
	fn path(&self) -> &'static Path;

	/// 返回配置内容
	///
	/// 配置的 TOML 值表示。
	fn config(&self) -> Value;
}

impl PartialEq for dyn Config {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name()
			&& self.path() == other.path()
			&& self.config() == other.config()
	}
}


use crate::Config;
use std::path::Path;
use std::sync::Arc;
use toml::Value;

/// 配置标识符
///
/// 用于在注册表中查询配置，支持通过索引或路径进行查询。
///
/// # 示例
///
/// ```rust
/// use puniyu_config_core::ConfigId;
/// use std::path::Path;
///
/// // 通过索引
/// let id: ConfigId = 0u64.into();
///
/// // 通过路径
/// let id: ConfigId = Path::new("config/app.toml").into();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigId<'c> {
    /// 通过索引标识
    Index(u64),
    /// 通过路径标识
    Path(&'c Path),
}

impl From<u64> for ConfigId<'_> {
    fn from(index: u64) -> Self {
        Self::Index(index)
    }
}

impl<'c> From<&'c Path> for ConfigId<'c> {
    fn from(path: &'c Path) -> Self {
        Self::Path(path)
    }
}

impl<'c> From<&'c str> for ConfigId<'c> {
    fn from(path: &'c str) -> Self {
        Self::Path(Path::new(path))
    }
}

/// 配置信息
///
/// 存储配置的元数据和内容。
///
/// # 字段
///
/// - `name`: 配置名称，用于标识配置
/// - `path`: 配置文件路径
/// - `value`: 配置的 TOML 值
///
/// # 示例
///
/// ```rust
/// use puniyu_config_core::ConfigInfo;
/// use std::path::Path;
/// use toml::Value;
///
/// let info = ConfigInfo {
///     name: "app",
///     path: Path::new("config/app.toml"),
///     value: toml::toml! {
///         version = "1.0.0"
///     },
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigInfo {
    /// 配置文件名称
    pub name: &'static str,
    /// 配置文件路径
    pub path: &'static Path,
    /// 配置内容
    pub value: Value,
}


impl From<Arc<dyn Config>> for ConfigInfo {
    fn from(value: Arc<dyn Config>) -> Self {
        Self {
            name: value.name(),
            path: value.path(),
            value: value.config(),
        }
    }
}

use crate::APP_NAME;
use std::{env::current_dir, path::PathBuf, sync::LazyLock};

/// 应用根目录
///
/// ## 示例
/// ```
/// use puniyu_utils::path::BASE_DIR;
/// let base_dir = BASE_DIR.as_path();
/// ```
pub static BASE_DIR: LazyLock<PathBuf> =
	LazyLock::new(|| current_dir().unwrap_or(PathBuf::from(".")));

/// 日志文件夹路径
///
/// # 示例
///
/// ```rust
/// use puniyu_utils::path::LOG_DIR;
/// let log_dir = LOG_DIR.as_path();
/// ```
pub static LOG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = BASE_DIR.to_path_buf();
	path.push("logs");
	path
});

/// 应用文件夹路径
/// 此目录存放数据，如插件数据, 适配器数据等
///
/// # 示例
///
/// ```rust
/// use puniyu_utils::path::APP_DIR;
/// let app_dir = APP_DIR.as_path();
/// ```
pub static APP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = BASE_DIR.to_path_buf();
	path.push(format!("@{}", APP_NAME.get().unwrap()));
	path
});

/// 配置文件夹路径
///
/// # 示例
///
/// ```
/// use puniyu_utils::path::CONFIG_DIR;
/// let config_dir = CONFIG_DIR.as_path();
/// ```
pub static CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = APP_DIR.to_path_buf();
	path.push("config");
	path
});

/// 临时文件夹路径
///
/// # 示例
///
/// ```
/// use puniyu_utils::path::TEMP_DIR;
/// let temp_dir = TEMP_DIR.as_path();
/// ```
pub static TEMP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = APP_DIR.to_path_buf();
	path.push("temp");
	path
});

/// 插件文件夹路径
///
/// # 示例
///
/// ```
/// use puniyu_utils::path::PLUGIN_DIR;
/// let plugin_dir = PLUGIN_DIR.as_path();
/// ```
pub static PLUGIN_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = BASE_DIR.to_path_buf();
	path.push("plugins");
	path
});

/// 适配器文件夹路径
///
/// # 示例
///
/// ```
/// use puniyu_utils::path::ADAPTER_DIR;
/// let adapter_dir = ADAPTER_DIR.as_path();
/// ```
pub static ADAPTER_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = BASE_DIR.to_path_buf();
	path.push("adapters");
	path
});

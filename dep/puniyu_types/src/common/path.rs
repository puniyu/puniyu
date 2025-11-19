use super::APP_NAME;
use std::sync::OnceLock;
use std::{env::current_dir, path::PathBuf, sync::LazyLock};

pub static WORKING_DIR: OnceLock<PathBuf> = OnceLock::new();

/// 应用根目录
///
/// ## 示例
/// ```
/// use puniyu_types::common::BASE_DIR;
/// let base_dir = BASE_DIR.as_path();
/// ```
pub static BASE_DIR: LazyLock<PathBuf> =
	LazyLock::new(|| PathBuf::from(WORKING_DIR.get_or_init(|| current_dir().unwrap())));

/// 应用文件夹路径
/// 此目录存放数据，如插件数据, 适配器数据等
///
/// # 示例
///
/// ```rust
/// use puniyu_types::common::APP_DIR;
/// let app_dir = APP_DIR.as_path();
/// ```
pub static APP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = BASE_DIR.to_path_buf();
	path.push(format!("@{}", APP_NAME.get_or_init(|| String::from("puniyu"))));
	path
});

/// 日志文件夹路径
///
/// # 示例
///
/// ```rust
/// use puniyu_types::common::LOG_DIR;
/// let log_dir = LOG_DIR.as_path();
/// ```
pub static LOG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = APP_DIR.to_path_buf();
	path.push("logs");
	path
});

/// 配置文件夹路径
///
/// 此目录会存放插件配置文件，适配器配置文件
/// # 示例
///
/// ```
/// use puniyu_types::common::CONFIG_DIR;
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
/// use puniyu_types::common::TEMP_DIR;
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
/// use puniyu_common::path::PLUGIN_DIR;
/// let plugin_dir = PLUGIN_DIR.as_path();
/// ```
pub static PLUGIN_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = BASE_DIR.to_path_buf();
	path.push("plugins");
	path
});

/// 缓存文件夹路径
///
/// # 示例
///
/// ```
/// use puniyu_common::path::DATA_DIR;
/// let data_dir = DATA_DIR.as_path();
/// ```
pub static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = APP_DIR.to_path_buf();
	path.push("data");
	path
});

/// 插件数据文件夹
///
/// # 示例
///
/// ```
/// use puniyu_common::path::PLUGIN_DATA_DIR;
/// let plugin_data_dir = PLUGIN_DATA_DIR.as_path();
/// ```
pub static PLUGIN_DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = DATA_DIR.to_path_buf();
	path.push("plugins");
	path
});

/// 适配器数据文件夹
///
/// # 示例
///
/// ```
/// use puniyu_common::path::ADAPTER_DATA_DIR;
/// let adapter_data_dir = ADAPTER_DATA_DIR.as_path();
/// ```
pub static ADAPTER_DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = DATA_DIR.to_path_buf();
	path.push("adapters");
	path
});

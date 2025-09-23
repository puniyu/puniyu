use std::{env::current_dir, path::PathBuf, sync::LazyLock};

static BASE_DIR: LazyLock<PathBuf> = LazyLock::new(|| current_dir().unwrap());

static PUNIYU_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = BASE_DIR.to_path_buf();
	path.push("@puniyu");
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
	let mut path = PUNIYU_DIR.to_path_buf();
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
	let mut path = PUNIYU_DIR.to_path_buf();
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

use std::path::PathBuf;
use std::sync::LazyLock;

pub(crate) const FOLDER_NAME: &str = "adapters";

/// 适配器配置文件夹路径
///
/// 存放适配器配置文件
///
/// # 示例
/// ```
/// use puniyu_common::path::ADAPTER_CONFIG_DIR;
/// let adapter_config_dir = ADAPTER_CONFIG_DIR.as_path();
/// ```
pub static CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = crate::CONFIG_DIR.to_path_buf();
	path.push(FOLDER_NAME);
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
pub static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = crate::DATA_DIR.to_path_buf();
	path.push(FOLDER_NAME);
	path
});

/// 适配器资源文件夹路径
///
/// 此目录存放适配器资源文件，如图片，字体等
///
/// # 示例
/// ```
/// use puniyu_common::path::ADAPTER_RESOURCE_DIR;
/// let adapter_resource_dir = ADAPTER_RESOURCE_DIR.as_path();
/// ```
pub static RESOURCE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = crate::RESOURCE_DIR.to_path_buf();
	path.push(FOLDER_NAME);
	path
});


/// 适配器临时文件夹路径
///
/// 存放适配器临时文件
///
/// ## 示例
/// ```
/// use puniyu_common::path::ADAPTER_TEMP_DIR;
/// let adapter_temp_dir = ADAPTER_TEMP_DIR.as_path();
/// ```
pub static TEMP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = crate::TEMP_DIR.to_path_buf();
	path.push(FOLDER_NAME);
	path
});

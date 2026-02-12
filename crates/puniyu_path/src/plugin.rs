use std::path::PathBuf;
use std::sync::LazyLock;

pub(crate) const FOLDER_NAME:&str = "plugins";

/// 插件配置文件夹路径
///
/// 存放插件配置文件
///
/// # 示例
/// ```
/// use puniyu_common::path::PLUGIN_CONFIG_DIR;
/// let plugin_config_dir = PLUGIN_CONFIG_DIR.as_path();
/// ```
pub static CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = crate::CONFIG_DIR.to_path_buf();
    path.push(FOLDER_NAME);
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
pub static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = crate::DATA_DIR.to_path_buf();
    path.push(FOLDER_NAME);
    path
});


/// 插件资源文件夹路径
///
/// 此目录存放插件资源文件，如图片，字体等
///
/// # 示例
/// ```
/// use puniyu_common::path::PLUGIN_RESOURCE_DIR;
/// let plugin_resource_dir = PLUGIN_RESOURCE_DIR.as_path();
/// ```
pub static RESOURCE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = crate::RESOURCE_DIR.to_path_buf();
    path.push("plugins");
    path
});


/// 插件临时文件夹路径
///
/// 存放插件临时文件
///
/// # 示例
/// ```
/// use puniyu_common::path::PLUGIN_TEMP_DIR;
/// let plugin_temp_dir = PLUGIN_TEMP_DIR.as_path();
/// ```
pub static TEMP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = crate::TEMP_DIR.to_path_buf();
    path.push(FOLDER_NAME);
    path
});
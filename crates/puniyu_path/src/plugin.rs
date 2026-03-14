use std::path::PathBuf;

pub(crate) const NAME:&str = "plugins";

/// 插件配置文件夹路径
///
/// 存放插件配置文件
///
/// # 示例
/// ```
/// use puniyu_path::plugin;
/// let plugin_config = plugin::config_dir();
/// ```
pub fn config_dir() -> PathBuf {
    crate::config_dir().join(NAME)
}

/// 插件数据文件夹
///
/// # 示例
///
/// ```
/// use puniyu_path::plugin;
/// let plugin_data = plugin::data_dir();
/// ```
pub fn data_dir() -> PathBuf {
    crate::data_dir().join(NAME)
}

/// 插件资源文件夹路径
///
/// 此目录存放插件资源文件，如图片，字体等
///
/// # 示例
/// ```
/// use puniyu_path::plugin;
/// let plugin_resource = plugin::resource_dir();
/// ```
pub fn resource_dir() -> PathBuf {
    crate::resource_dir().join(NAME)
}

/// 插件临时文件夹路径
///
/// 存放插件临时文件
///
/// # 示例
/// ```
/// use puniyu_path::plugin;
/// let plugin_temp = plugin::temp_dir();
/// ```
pub fn temp_dir() -> PathBuf {
    crate::temp_dir().join(NAME)
}
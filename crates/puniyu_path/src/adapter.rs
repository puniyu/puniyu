use std::path::PathBuf;

pub(crate) const NAME: &str = "adapters";

/// 适配器配置文件夹路径
///
/// 存放适配器配置文件
///
/// # 示例
/// ```
/// use puniyu_path::adapter;
/// let adapter_config = adapter::config_dir();
/// ```
pub fn config_dir() -> PathBuf {
	crate::config_dir().join(NAME)
}

/// 适配器数据文件夹
///
/// # 示例
///
/// ```
/// use puniyu_path::adapter;
/// let adapter_data = adapter::data_dir();
/// ```
pub fn data_dir() -> PathBuf {
	crate::data_dir().join(NAME)
}

/// 适配器资源文件夹路径
///
/// 此目录存放适配器资源文件，如图片，字体等
///
/// # 示例
/// ```
/// use puniyu_path::adapter;
/// let adapter_resource = adapter::resource_dir();
/// ```
pub fn resource_dir() -> PathBuf {
	crate::resource_dir().join(NAME)
}

/// 适配器临时文件夹路径
///
/// 存放适配器临时文件
///
/// # 示例
/// ```
/// use puniyu_path::adapter;
/// let adapter_temp = adapter::temp_dir();
/// ```
pub fn temp_dir() -> PathBuf {
	crate::temp_dir().join(NAME)
}

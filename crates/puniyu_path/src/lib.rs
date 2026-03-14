pub mod adapter;
pub mod plugin;

use puniyu_common::app::{app_name, app_working_dir};
use std::path::PathBuf;

/// 应用根目录
///
/// 等同于工作目录，是所有其他目录的基础路径。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::base_dir;
///
/// let base = base_dir();
/// println!("Base directory: {:?}", base);
/// ```
pub fn base_dir() -> PathBuf {
	app_working_dir().to_path_buf()
}

/// 应用文件夹路径
///
/// 此目录存放应用数据，如插件数据、适配器数据等。
/// 路径格式为 `{BASE_DIR}/@{APP_NAME}`。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::app_dir;
///
/// let app = app_dir();
/// println!("App directory: {:?}", app);
/// ```
pub fn app_dir() -> PathBuf {
	let name = format!("@{name}", name = app_name());
	base_dir().join(name).to_path_buf()
}

/// 日志文件夹路径
///
/// 存放应用日志文件。
/// 路径格式为 `{APP_DIR}/logs`。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::log_dir;
///
/// let log = log_dir();
/// println!("Log directory: {:?}", log);
/// ```
pub fn log_dir() -> PathBuf {
	base_dir().join("logs").to_path_buf()
}

/// 配置文件夹路径
///
/// 此目录存放插件配置文件、适配器配置文件等。
/// 路径格式为 `{APP_DIR}/config`。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::config_dir;
///
/// let config = config_dir();
/// println!("Config directory: {:?}", config);
/// ```
pub fn config_dir() -> PathBuf {
	base_dir().join("config").to_path_buf()
}

/// 临时文件夹路径
///
/// 存放临时文件。
/// 路径格式为 `{APP_DIR}/temp`。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::temp_dir;
///
/// let temp = temp_dir();
/// println!("Temp directory: {:?}", temp);
/// ```
pub fn temp_dir() -> PathBuf {
	base_dir().join("temp").to_path_buf()
}

/// 插件文件夹路径
///
/// 存放插件文件。
/// 路径格式为 `{BASE_DIR}/plugins`。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::plugin_dir;
///
/// let plugin = plugin_dir();
/// println!("Plugin directory: {:?}", plugin);
/// ```
pub fn plugin_dir() -> PathBuf {
	base_dir().join("plugins").to_path_buf()
}

/// 适配器文件夹路径
///
/// 存放适配器文件。
/// 路径格式为 `{BASE_DIR}/adapters`。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::adapter_dir;
///
/// let adapter = adapter_dir();
/// println!("Adapter directory: {:?}", adapter);
/// ```
pub fn adapter_dir() -> PathBuf {
	base_dir().join("adapters").to_path_buf()
}

/// 数据文件夹路径
///
/// 存放应用数据文件。
/// 路径格式为 `{APP_DIR}/data`。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::data_dir;
///
/// let data = data_dir();
/// println!("Data directory: {:?}", data);
/// ```
pub fn data_dir() -> PathBuf {
	base_dir().join("data").to_path_buf()
}

/// 资源文件夹路径
///
/// 此目录存放资源文件，如图片、字体等。
/// 路径格式为 `{APP_DIR}/resources`。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::resource_dir;
///
/// let resource = resource_dir();
/// println!("Resource directory: {:?}", resource);
/// ```
pub fn resource_dir() -> PathBuf {
	base_dir().join("resources").to_path_buf()
}
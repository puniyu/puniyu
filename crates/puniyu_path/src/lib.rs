//! # puniyu_path
//!
//! 路径管理库，提供应用程序各种目录路径的统一管理。
//!
//! ## 概述
//!
//! `puniyu_path` 提供了应用程序运行时所需的各种目录路径，包括配置目录、数据目录、
//! 日志目录、插件目录、适配器目录等。所有路径都是基于工作目录和应用名称动态生成的。
//!
//! ## 使用方式
//!
//! ### 初始化
//!
//! 在使用任何路径之前，必须先初始化 `WORKING_DIR` 和 `APP_NAME`：
//!
//! ```rust
//! use puniyu_path::WORKING_DIR;
//! use puniyu_common::APP_NAME;
//! use std::path::PathBuf;
//!
//! // 初始化工作目录
//! WORKING_DIR.set(PathBuf::from("/path/to/work")).ok();
//!
//! // 初始化应用名称
//! APP_NAME.set("myapp".to_string()).ok();
//! ```
//!
//! ### 使用路径
//!
//! ```rust
//! use puniyu_path::{BASE_DIR, APP_DIR, CONFIG_DIR, LOG_DIR};
//!
//! // 获取基础目录
//! let base = BASE_DIR.as_path();
//!
//! // 获取应用目录
//! let app = APP_DIR.as_path();
//!
//! // 获取配置目录
//! let config = CONFIG_DIR.as_path();
//!
//! // 获取日志目录
//! let log = LOG_DIR.as_path();
//! ```

pub mod adapter;
pub mod plugin;

use puniyu_common::APP_NAME;
use std::sync::OnceLock;
use std::{path::PathBuf, sync::LazyLock};

/// 应用工作目录
///
/// 不推荐直接使用，请使用 [`BASE_DIR`] 代替。
///
/// # 注意
///
/// 必须在程序启动时初始化此变量，否则访问其他路径时会 panic。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::WORKING_DIR;
/// use std::path::PathBuf;
///
/// WORKING_DIR.set(PathBuf::from("/path/to/work")).ok();
/// ```
pub static WORKING_DIR: OnceLock<PathBuf> = OnceLock::new();

/// 应用根目录
///
/// 等同于工作目录，是所有其他目录的基础路径。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::BASE_DIR;
///
/// let base_dir = BASE_DIR.as_path();
/// println!("Base directory: {:?}", base_dir);
/// ```
pub static BASE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	#[allow(clippy::unwrap_used)]
	let path = WORKING_DIR.get().unwrap(); // 这个WORKING_DIR会在整个程序运行前就初始化的, 这里的unwrap 是安全的
	path.to_path_buf()
});

/// 应用文件夹路径
///
/// 此目录存放应用数据，如插件数据、适配器数据等。
/// 路径格式为 `{BASE_DIR}/@{APP_NAME}`。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::APP_DIR;
///
/// let app_dir = APP_DIR.as_path();
/// println!("App directory: {:?}", app_dir);
/// ```
pub static APP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = BASE_DIR.to_path_buf();
	#[allow(clippy::unwrap_used)]
	let name = APP_NAME.get().unwrap(); // 这个APP_NAME是在整个程序运行前就初始化的, 这里的unwrap 是安全的
	path.push(format!("@{}", name));
	path
});

/// 日志文件夹路径
///
/// # 示例
///
/// ```rust
/// use puniyu_path::LOG_DIR;
/// let log_dir = LOG_DIR.as_path();
/// ```
/// 日志文件夹路径
///
/// 存放应用日志文件。
/// 路径格式为 `{APP_DIR}/logs`。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::LOG_DIR;
///
/// let log_dir = LOG_DIR.as_path();
/// println!("Log directory: {:?}", log_dir);
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
/// use puniyu_path::CONFIG_DIR;
/// let config_dir = CONFIG_DIR.as_path();
/// ```
/// 配置文件夹路径
///
/// 此目录存放插件配置文件、适配器配置文件等。
/// 路径格式为 `{APP_DIR}/config`。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::CONFIG_DIR;
///
/// let config_dir = CONFIG_DIR.as_path();
/// println!("Config directory: {:?}", config_dir);
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
/// use puniyu_path::TEMP_DIR;
/// let temp_dir = TEMP_DIR.as_path();
/// ```
/// 临时文件夹路径
///
/// 存放临时文件。
/// 路径格式为 `{APP_DIR}/temp`。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::TEMP_DIR;
///
/// let temp_dir = TEMP_DIR.as_path();
/// println!("Temp directory: {:?}", temp_dir);
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
/// ```rust
/// use puniyu_path::PLUGIN_DIR;
/// let plugin_dir = PLUGIN_DIR.as_path();
/// ```
/// 插件文件夹路径
///
/// 存放插件文件。
/// 路径格式为 `{BASE_DIR}/plugins`。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::PLUGIN_DIR;
///
/// let plugin_dir = PLUGIN_DIR.as_path();
/// println!("Plugin directory: {:?}", plugin_dir);
/// ```
pub static PLUGIN_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = BASE_DIR.to_path_buf();
	path.push(plugin::FOLDER_NAME);
	path
});

/// 适配器文件夹路径
///
/// # 示例
///
/// ```
/// use puniyu_path::ADAPTER_DIR;
/// let plugin_dir = PLUGIN_DIR.as_path();
/// ```
/// 适配器文件夹路径
///
/// 存放适配器文件。
/// 路径格式为 `{BASE_DIR}/adapters`。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::ADAPTER_DIR;
///
/// let adapter_dir = ADAPTER_DIR.as_path();
/// println!("Adapter directory: {:?}", adapter_dir);
/// ```
pub static ADAPTER_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = BASE_DIR.to_path_buf();
	path.push(adapter::FOLDER_NAME);
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
/// 数据文件夹路径
///
/// 存放应用数据文件。
/// 路径格式为 `{APP_DIR}/data`。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::DATA_DIR;
///
/// let data_dir = DATA_DIR.as_path();
/// println!("Data directory: {:?}", data_dir);
/// ```
pub static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = APP_DIR.to_path_buf();
	path.push("data");
	path
});

/// 资源文件夹路径
///
/// 此目录存放插件资源文件，如图片，字体等
///
/// # 示例
/// ```
/// use puniyu_common::path::RESOURCE_DIR;
/// let resource_dir = RESOURCE_DIR.as_path();
/// ```
/// 资源文件夹路径
///
/// 此目录存放资源文件，如图片、字体等。
/// 路径格式为 `{APP_DIR}/resources`。
///
/// # 示例
///
/// ```rust
/// use puniyu_path::RESOURCE_DIR;
///
/// let resource_dir = RESOURCE_DIR.as_path();
/// println!("Resource directory: {:?}", resource_dir);
/// ```
pub static RESOURCE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = APP_DIR.to_path_buf();
	path.push("resources");
	path
});

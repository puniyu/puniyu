mod error;
pub use error::Error;
pub mod path;
pub use path::*;
pub mod toml;
pub use toml::{delete_config, merge_config, read_config, update_config, write_config};
pub mod system;

/// TODO: 获取插件指定插件的配置文件
use std::sync::OnceLock;
/// 当前程序名称, 也就是此实例的名称
///
/// ## 示例
/// ```rust
/// use puniyu_types::common::APP_NAME;
/// let name = APP_NAME.get_or_init(|| String::from("puniyu"));
/// ```
pub static APP_NAME: OnceLock<String> = OnceLock::new();

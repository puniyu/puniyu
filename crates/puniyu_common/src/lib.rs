mod config;
#[doc(inline)]
pub use config::{delete_config, merge_config, read_config, update_config, write_config};

#[cfg(feature = "version")]
mod version;

#[cfg(feature = "source")]
pub mod source;

#[cfg(feature = "version")]
pub use version::{Channel, VERSION, Version};

use std::sync::OnceLock;
/// 当前应用名称, 也就是此实例的名称
///
/// ## 示例
/// ```rust
/// use puniyu_common::APP_NAME;
/// let name = APP_NAME.get_or_init(|| String::from("puniyu"));
/// ```
pub static APP_NAME: OnceLock<String> = OnceLock::new();

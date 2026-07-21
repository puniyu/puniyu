use std::path::PathBuf;

use convert_case::{Case, Casing};

pub(crate) const NAME: &str = "plugins";

/// 获取插件配置目录，格式为 `{CONFIG_DIR}/plugins/{plugin_name}`。
pub fn config_dir(plugin_name: &str) -> PathBuf {
	crate::config_dir().join(NAME).join(plugin_name.to_case(Case::Kebab))
}

/// 获取插件数据目录，格式为 `{DATA_DIR}/plugins/{plugin_name}`。
pub fn data_dir(plugin_name: &str) -> PathBuf {
	crate::data_dir().join(NAME).join(plugin_name.to_case(Case::Kebab))
}

/// 获取插件资源目录，格式为 `{ASSETS_DIR}/plugins/{plugin_name}`。
pub fn assets_dir(plugin_name: &str) -> PathBuf {
	crate::assets_dir().join(NAME).join(plugin_name.to_case(Case::Kebab))
}

/// 获取插件临时目录，格式为 `{TEMP_DIR}/plugins/{plugin_name}`。
pub fn temp_dir(plugin_name: &str) -> PathBuf {
	crate::temp_dir().join(NAME).join(plugin_name.to_case(Case::Kebab))
}

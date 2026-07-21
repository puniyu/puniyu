use std::path::PathBuf;

use convert_case::{Case, Casing};

pub(crate) const NAME: &str = "adapters";

/// 获取适配器配置目录，格式为 `{CONFIG_DIR}/adapters/{adapter_name}`。
pub fn config_dir(adapter_name: &str) -> PathBuf {
	crate::config_dir().join(NAME).join(adapter_name.to_case(Case::Kebab))
}

/// 获取适配器数据目录，格式为 `{DATA_DIR}/adapters/{adapter_name}`。
pub fn data_dir(adapter_name: &str) -> PathBuf {
	crate::data_dir().join(NAME).join(adapter_name.to_case(Case::Kebab))
}

/// 获取适配器资源目录，格式为 `{ASSETS_DIR}/adapters/{adapter_name}`。
pub fn assets_dir(adapter_name: &str) -> PathBuf {
	crate::assets_dir().join(NAME).join(adapter_name.to_case(Case::Kebab))
}

/// 获取适配器临时目录，格式为 `{TEMP_DIR}/adapters/{adapter_name}`。
pub fn temp_dir(adapter_name: &str) -> PathBuf {
	crate::temp_dir().join(NAME).join(adapter_name.to_case(Case::Kebab))
}

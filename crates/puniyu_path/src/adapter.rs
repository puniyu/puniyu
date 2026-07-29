use std::path::PathBuf;

use smol_str::SmolStr;

/// 适配器路径
pub struct Path {
	pub(crate) name: SmolStr,
	pub(crate) base_dir: PathBuf,
}

impl Path {
	/// 适配器配置目录：`{base_dir}/config/adapters/{name}`
	pub fn config_dir(&self) -> PathBuf {
		self.base_dir.join("config").join("adapters").join(&self.name)
	}

	/// 适配器数据目录：`{base_dir}/data/adapters/{name}`
	pub fn data_dir(&self) -> PathBuf {
		self.base_dir.join("data").join("adapters").join(&self.name)
	}

	/// 适配器资源目录：`{base_dir}/assets/adapters/{name}`
	pub fn assets_dir(&self) -> PathBuf {
		self.base_dir.join("assets").join("adapters").join(&self.name)
	}

	/// 适配器临时目录：`{base_dir}/temp/adapters/{name}`
	pub fn temp_dir(&self) -> PathBuf {
		self.base_dir.join("temp").join("adapters").join(&self.name)
	}
}

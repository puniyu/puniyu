use std::path::PathBuf;
use smol_str::SmolStr;

pub struct Path {
	pub(crate) name: SmolStr,
	pub(crate) base_dir: PathBuf,
}

impl Path {
	/// 插件配置目录：`{base_dir}/config/plugins/{name}`
	pub fn config_dir(&self) -> PathBuf {
		self.base_dir.join("config").join("plugins").join(&self.name)
	}

	/// 插件数据目录：`{base_dir}/data/plugins/{name}`
	pub fn data_dir(&self) -> PathBuf {
		self.base_dir.join("data").join("plugins").join(&self.name)
	}

	/// 插件资源目录：`{base_dir}/assets/plugins/{name}`
	pub fn assets_dir(&self) -> PathBuf {
		self.base_dir.join("assets").join("plugins").join(&self.name)
	}

	/// 插件临时目录：`{base_dir}/temp/plugins/{name}`
	pub fn temp_dir(&self) -> PathBuf {
		self.base_dir.join("temp").join("plugins").join(&self.name)
	}
}

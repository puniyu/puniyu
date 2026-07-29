pub mod plugin;
pub mod adapter;

use convert_case::{Case, Casing};
use std::path::PathBuf;
use sugar_path::SugarPath;

use smol_str::SmolStr;

/// 应用路径管理器。
///
/// 基于 `cwd_dir` 和 `name` 构建统一的目录结构：
/// ```text
/// {cwd_dir}/
/// └── {name}/
///     ├── logs/
///     ├── config/
///     ├── data/
///     ├── assets/
///     ├── temp/
///     ├── plugins/
///     └── adapters/
/// ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
	name: SmolStr,
	cwd_dir: PathBuf,
}

impl Path {
	/// 创建路径管理器。
	///
	/// - `name`：应用名称，作为基础目录名
	/// - `base_dir`：工作目录
	pub fn new(name: impl Into<SmolStr>, base_dir: impl Into<PathBuf>) -> Self {
		Self { 
			name: name.into(), 
			cwd_dir: PathBuf::from(base_dir.into().to_slash().as_ref()) 
		}
	}

	/// 工作目录。
	pub fn cwd_dir(&self) -> PathBuf {
		self.cwd_dir.clone()
	}

	/// 基础目录：`{cwd_dir}/{name}`
	pub fn base_dir(&self) -> PathBuf {
		self.cwd_dir.join(&self.name)
	}

	/// 日志目录：`{base_dir}/logs`
	pub fn log_dir(&self) -> PathBuf {
		self.base_dir().join("logs")
	}

	/// 配置目录：`{base_dir}/config`
	pub fn config_dir(&self) -> PathBuf {
		self.base_dir().join("config")
	}

	/// 数据目录：`{base_dir}/data`
	pub fn data_dir(&self) -> PathBuf {
		self.base_dir().join("data")
	}

	/// 资源目录：`{base_dir}/assets`
	pub fn assets_dir(&self) -> PathBuf {
		self.base_dir().join("assets")
	}

	/// 临时目录：`{base_dir}/temp`
	pub fn temp_dir(&self) -> PathBuf {
		self.base_dir().join("temp")
	}

	/// 插件根目录：`{base_dir}/plugins`
	pub fn plugins_dir(&self) -> PathBuf {
		self.base_dir().join("plugins")
	}

	/// 适配器根目录：`{base_dir}/adapters`
	pub fn adapters_dir(&self) -> PathBuf {
		self.base_dir().join("adapters")
	}

	/// 获取插件子路径
	pub fn plugin(&self, name: &str) -> plugin::Path {
		plugin::Path {
			name: SmolStr::new(name.to_case(Case::Kebab)),
			base_dir: self.base_dir(),
		}
	}

	/// 获取适配器子路径
	pub fn adapter(&self, name: &str) -> adapter::Path {
		adapter::Path {
			name: SmolStr::new(name.to_case(Case::Kebab)),
			base_dir: self.base_dir(),
		}
	}
}

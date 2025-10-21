use crate::task::TaskBuilder;
use async_trait::async_trait;
use puniyu_command::CommandBuilder;
use std::fmt;
use std::path::PathBuf;

#[async_trait]
pub trait PluginBuilder: Send + Sync + 'static {
	/// 插件名称
	fn name(&self) -> &'static str;
	/// 插件版本
	fn version(&self) -> &'static str;

	/// api版本
	fn abi_version(&self) -> &'static str;

	// fn description(&self) -> &'static str;
	/// 插件作者
	fn author(&self) -> &'static str;

	/// 任务列表
	fn tasks(&self) -> Vec<Box<dyn TaskBuilder>>;

	/// 命令列表
	fn commands(&self) -> Vec<Box<dyn CommandBuilder>>;
	/// 插件初始化函数
	async fn init(&self) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug, Clone, Default)]
pub struct Plugin {
	/// 插件名称
	pub name: &'static str,
	/// 插件版本
	pub version: &'static str,
	/// 插件作者
	pub author: &'static str,
}

#[derive(Clone)]
pub enum PluginType {
	/// 基于文件路径加载的动态库插件
	Path(PathBuf),
	/// 静态链接的插件
	Builder(&'static dyn PluginBuilder),
}

impl fmt::Debug for PluginType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			PluginType::Path(path) => f.debug_struct("Path").field("path", path).finish(),
			PluginType::Builder(_) => f.debug_struct("Builder").finish(),
		}
	}
}
impl From<PathBuf> for PluginType {
	fn from(path: PathBuf) -> Self {
		PluginType::Path(path)
	}
}

impl From<&'static dyn PluginBuilder> for PluginType {
	fn from(builder: &'static dyn PluginBuilder) -> Self {
		PluginType::Builder(builder)
	}
}

#[derive(Debug, Clone)]
pub enum PluginId {
	Index(u64),
	Name(String),
}
impl From<u64> for PluginId {
	fn from(value: u64) -> Self {
		Self::Index(value)
	}
}
impl From<String> for PluginId {
	fn from(value: String) -> Self {
		Self::Name(value)
	}
}
impl From<&str> for PluginId {
	fn from(value: &str) -> Self {
		Self::Name(value.to_string())
	}
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

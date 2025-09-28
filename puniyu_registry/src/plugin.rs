use crate::plugin::{builder::PluginBuilder, command::Command, manger::PluginInfo, task::Task};

pub mod builder;
pub mod command;
pub mod manger;
pub mod registry;
pub mod task;

#[derive(Debug, Clone)]
pub struct Plugin {
	pub info: PluginInfo,
	pub tasks: Vec<Task>,
	pub commands: Vec<Command>,
}

/// 定义插件类型枚举
#[derive(Clone)]
pub enum PluginType {
	/// 基于文件路径加载的动态库插件
	Path(String),
	/// 静态链接的插件
	Builder(&'static dyn PluginBuilder),
}

impl From<&str> for PluginType {
	fn from(path: &str) -> Self {
		PluginType::Path(path.to_string())
	}
}

impl From<String> for PluginType {
	fn from(path: String) -> Self {
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

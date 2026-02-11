mod types;
#[doc(inline)]
pub use types::*;
#[cfg(feature = "registry")]
mod registry;
#[cfg(feature = "registry")]
pub use registry::PluginRegistry;

use std::sync::Arc;
use async_trait::async_trait;
use puniyu_command::Command;
use puniyu_config::types::Config;
use puniyu_hook::Hook;
use puniyu_task::Task;
use puniyu_error::Result;
use puniyu_server_core::ServerFunction;
use puniyu_version::Version;


#[async_trait]
pub trait Plugin: Send + Sync + 'static {
	/// 插件名称
	fn name(&self) -> &'static str;
	/// 插件版本
	fn version(&self) -> &Version;

	/// api版本
	fn abi_version(&self) -> &Version;

	/// 插件描述
	fn description(&self) -> Option<&str>;
	/// 插件作者
	fn author(&self) -> Option<&str> {
		None
	}

	/// 插件命令前缀
	fn prefix(&self) -> Option<&str> {
		None
	}

	/// 任务列表
	fn tasks(&self) -> Vec<Arc<dyn Task>> {
		Vec::new()
	}

	/// 命令列表
	fn commands(&self) -> Vec<Arc<dyn Command>> {
		Vec::new()
	}

	/// 钩子列表
	fn hooks(&self) -> Vec<Arc<dyn Hook>> {
		Vec::new()
	}

	/// 插件配置文件
	fn config(&self) -> Vec<Arc<dyn Config>> {
		Vec::new()
	}

	/// 路由管理
	fn server(&self) -> Option<ServerFunction> {
		None
	}
	/// 插件初始化函数
	async fn init(&self) -> Result;
}

impl PartialEq for dyn Plugin {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name()
	}
}
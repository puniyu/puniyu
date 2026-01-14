use crate::command::CommandBuilder;
use crate::config::Config;
use crate::handler::HandlerResult;
use crate::hook::HookBuilder;
use crate::server::ServerType;
use crate::task::TaskBuilder;
use crate::version::Version;
use async_trait::async_trait;

#[async_trait]
pub trait PluginBuilder: Send + Sync {
	/// 插件名称
	fn name(&self) -> &str;
	/// 插件版本
	fn version(&self) -> Version;

	/// api版本
	fn abi_version(&self) -> Version;

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
	fn tasks(&self) -> Vec<Box<dyn TaskBuilder>> {
		Vec::new()
	}

	/// 命令列表
	fn commands(&self) -> Vec<Box<dyn CommandBuilder>> {
		Vec::new()
	}

	/// 钩子列表
	fn hooks(&self) -> Vec<Box<dyn HookBuilder>> {
		Vec::new()
	}

	/// 插件配置文件
	fn config(&self) -> Vec<Box<dyn Config>> {
		Vec::new()
	}

	/// 路由管理
	fn server(&self) -> Option<ServerType> {
		None
	}
	/// 插件初始化函数
	async fn init(&self) -> HandlerResult;
}

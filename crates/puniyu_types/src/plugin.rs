use crate::command::CommandBuilder;
use crate::config::Config;
use crate::server::ServerType;
use crate::task::TaskBuilder;
use crate::version::Version;
use async_trait::async_trait;

#[async_trait]
pub trait PluginBuilder: Send + Sync + 'static {
	/// 插件名称
	fn name(&self) -> &'static str;
	/// 插件版本
	fn version(&self) -> Version;

	/// api版本
	fn abi_version(&self) -> Version;

	/// 插件描述
	fn description(&self) -> &'static str;
	/// 插件作者
	fn author(&self) -> Option<&'static str>;

	/// 插件命令前缀
	fn prefix(&self) -> Option<&'static str> {
		None
	}

	/// 任务列表
	fn tasks(&self) -> Vec<Box<dyn TaskBuilder>>;

	/// 命令列表
	fn commands(&self) -> Vec<Box<dyn CommandBuilder>>;

	/// 插件配置文件
	fn config(&self) -> Option<Vec<Box<dyn Config>>> {
		None
	}

	/// 路由管理
	fn server(&self) -> Option<ServerType> {
		None
	}
	/// 插件初始化函数
	async fn init(&self) -> Result<(), Box<dyn std::error::Error>>;
}

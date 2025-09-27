use super::{command::builder::CommandBuilder, task::builder::TaskBuilder};
use async_trait::async_trait;

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
	async fn init(&self);
}

mod account;

pub use account::*;
mod info;
pub use info::*;

use async_trait::async_trait;
pub use puniyu_adapter_api::AdapterApi;
use puniyu_logger::info;

#[derive(Clone)]
pub struct Adapter {
	pub info: AdapterInfo,
	pub api: &'static dyn AdapterApi,
}

/// 适配器基类
/// 开发者需要自行实现开发适配器，部分函数需要开发者自行实现
///
#[async_trait]
pub trait AdapterBuilder: Send + Sync + 'static {
	/// 适配器信息
	fn info(&self) -> AdapterInfo;

	/// 获取适配器API
	fn api(&self) -> &'static dyn AdapterApi;

	/// 路由管理
	/// 默认返回 None，如果需要 HTTP 服务器可以重写此方法
	fn server(&self) -> Option<crate::ServerType> {
		None
	}

	/// 初始化
	async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
		info!("适配器: {} 初始化完成", self.info().name);
		Ok(())
	}
}

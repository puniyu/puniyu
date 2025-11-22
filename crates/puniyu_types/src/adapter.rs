pub mod types;
pub use types::*;
mod error;
pub use error::Error;
mod api;
pub use api::AdapterApi;
mod info;
pub use info::*;

use async_trait::async_trait;
use puniyu_logger::info;
use crate::{config::Config, server::ServerType};


pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct Adapter {
	pub info: AdapterInfo,
	pub api: &'static dyn AdapterApi,
}


/// 适配器构建器
#[async_trait]
pub trait AdapterBuilder: Send + Sync + 'static {
	/// 适配器信息
	fn info(&self) -> AdapterInfo;

	/// 获取适配器API
	fn api(&self) -> &'static dyn AdapterApi;

	fn config(&self) -> Option<Vec<Box<dyn Config>>> {
		None
	}

	/// 路由管理
	fn server(&self) -> Option<ServerType> {
		None
	}

	/// 初始化
	async fn init(&self) -> Result<()> {
		info!("适配器: {} 初始化完成", self.info().name);
		Ok(())
	}
}

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
use crate::version::Version;

pub type Result<T> = std::result::Result<T, Error>;

/// 适配器（registry 层面的存储结构）
#[derive(Clone)]
pub struct Adapter {
	/// 适配器名称
	pub name: String,
	/// 适配器版本
	pub version: String,
	/// 适配器 API
	pub api: &'static dyn AdapterApi,
}


/// 适配器构建器
#[async_trait]
pub trait AdapterBuilder: Send + Sync + 'static {
	/// 适配器名称
	fn name(&self) -> &'static str;

	/// 适配器版本
	fn version(&self) -> Version {
		Version::from(env!("CARGO_PKG_VERSION"))
	}

	/// 获取适配器API
	fn api(&self) -> &'static dyn AdapterApi;

	/// 配置文件
	fn config(&self) -> Option<Vec<Box<dyn Config>>> {
		None
	}

	/// 路由管理
	fn server(&self) -> Option<ServerType> {
		None
	}

	/// 初始化
	async fn init(&self) -> Result<()> {
		info!("适配器: {} v{} 初始化完成", self.name(), self.version());
		Ok(())
	}
}

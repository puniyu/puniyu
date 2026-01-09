pub mod types;
pub use types::*;
mod error;
pub use error::Error;
mod api;
pub use api::*;
mod info;
pub use info::*;

use crate::hook::HookBuilder;
use crate::version::Version;
use crate::{config::Config, server::ServerType};
use async_trait::async_trait;
use puniyu_logger::info;

pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
pub trait AdapterBuilder: Send + Sync {
	/// 适配器名称
	fn name(&self) -> &str;

	fn author(&self) -> Option<&str> {
		let authors = env!("CARGO_PKG_AUTHORS");
		if authors.is_empty() { None } else { Some(authors.split(';').next().unwrap_or(authors)) }
	}

	/// 适配器版本
	fn version(&self) -> Version {
		Version::from(env!("CARGO_PKG_VERSION"))
	}

	/// 获取适配器API
	fn api(&self) -> AdapterApi;

	/// 配置文件
	fn config(&self) -> Vec<Box<dyn Config>> {
		Vec::new()
	}

	/// 钩子列表
	fn hooks(&self) -> Vec<Box<dyn HookBuilder>> {
		Vec::new()
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

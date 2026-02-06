pub mod types;

use actix_web::web::ServiceConfig;
pub use types::*;
mod error;
pub use error::Error;
mod api;
pub use api::*;
mod info;
pub use info::*;

use crate::handler::HandlerResult;
use crate::hook::Hook;
use crate::{config::Config, server::ServerFunction};
use async_trait::async_trait;
use puniyu_logger::info;

pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
pub trait Plugin: Send + Sync {
	/// 适配器信息
	fn info(&self) -> AdapterInfo;
	/// 获取适配器API
	fn api(&self) -> AdapterApi {
		Default::default()
	}

	/// 配置文件
	fn config(&self) -> Vec<Box<dyn Config>> {
		Vec::new()
	}

	/// 钩子列表
	fn hooks(&self) -> Vec<Box<dyn Hook>> {
		Vec::new()
	}

	/// 路由管理
	fn server(&self) -> Option<ServerFunction> {
		None
	}

	/// 初始化
	async fn init(&self) -> HandlerResult {
		let (name, version) = (self.info().name, self.info().version);
		info!("适配器: {} v{} 初始化完成", name, version);
		Ok(())
	}
}

impl PartialEq for dyn Plugin {
	fn eq(&self, other: &Self) -> bool {
		self.info() == other.info()
	}
}

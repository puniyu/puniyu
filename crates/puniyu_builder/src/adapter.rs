mod account;
pub use account::*;
mod info;
pub use info::*;

use async_trait::async_trait;
pub use puniyu_adapter_api::AdapterApi;
use std::fmt;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Adapter {
	pub index: u64,
	pub info: AdapterInfo,
	pub api: &'static dyn AdapterApi,
}

#[derive(Clone)]
pub enum AdapterType {
	Path(PathBuf),
	Builder(&'static dyn AdapterBuilder),
}

impl fmt::Debug for AdapterType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			AdapterType::Path(path) => f.debug_struct("Path").field("path", path).finish(),
			AdapterType::Builder(_) => f.debug_struct("Builder").finish(),
		}
	}
}

impl From<PathBuf> for AdapterType {
	fn from(path: PathBuf) -> Self {
		AdapterType::Path(path)
	}
}

impl From<&'static dyn AdapterBuilder> for AdapterType {
	fn from(builder: &'static dyn AdapterBuilder) -> Self {
		AdapterType::Builder(builder)
	}
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

	/// 适配器ABI版本
	fn abi_version(&self) -> &'static str;

	/// 初始化
	async fn init(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

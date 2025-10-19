mod account;
mod types;
pub use account::AccountInfo;
mod info;
mod version;
pub use version::VERSION;

pub use info::{
	AdapterCommunication, AdapterInfo, AdapterPlatform, AdapterProtocol, AdapterStandard,
};
use std::fmt;
use std::path::PathBuf;

pub use types::*;

use async_trait::async_trait;
use puniyu_element::Message;
use puniyu_event_utils::contact::Contact;

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

pub trait AdapterApi: Send + Sync + 'static {
	fn send_msg(&self, contact: Contact, element: Message);
	/// 获取头像URL
	fn get_avatar_url(&self, user_id: &str, size: AvatarSize) -> String;

	/// 获取群头像URL
	fn get_group_avatar_url(&self, group_id: &str, size: AvatarSize) -> String;
}

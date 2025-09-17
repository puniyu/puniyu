pub mod manger;
mod registry;

pub use puniyu_utils::adapter::{
	AccountInfo, Adapter, AdapterApi, AdapterBase as AdapterBuilder, AdapterCommunication,
	AdapterInfo, AdapterPlatform, AdapterProtocol, AdapterStandard, AvatarSize,
};
use std::pin::Pin;

pub type AdapterFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

pub enum AdapterType {
	Path(String),
	Builder(&'static dyn AdapterBuilder),
}

impl From<&str> for AdapterType {
	fn from(path: &str) -> Self {
		AdapterType::Path(path.to_string())
	}
}
impl From<String> for AdapterType {
	fn from(path: String) -> Self {
		AdapterType::Path(path)
	}
}
impl From<&'static dyn AdapterBuilder> for AdapterType {
	fn from(builder: &'static dyn AdapterBuilder) -> Self {
		AdapterType::Builder(builder)
	}
}

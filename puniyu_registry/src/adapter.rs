mod builder;
pub(crate) mod registry;

pub use builder::AdapterBuilder;
use std::fmt;
mod store;

pub use puniyu_utils::adapter::{
	AccountInfo, Adapter, AdapterApi, AdapterCommunication, AdapterInfo, AdapterPlatform,
	AdapterProtocol, AdapterStandard, AvatarSize,
};
use std::path::PathBuf;
use std::pin::Pin;

pub type AdapterFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

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

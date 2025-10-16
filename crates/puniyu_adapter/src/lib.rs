mod account;

pub use account::AccountInfo;
use std::fmt;
use std::path::PathBuf;
mod builder;
pub use builder::{AdapterApi, AdapterBuilder, AvatarSize};
mod info;
pub use info::{
	AdapterCommunication, AdapterInfo, AdapterPlatform, AdapterProtocol, AdapterStandard,
};

mod error;
pub use error::Error;
mod registry;
pub use registry::AdapterRegistry;
mod types;
pub use types::*;
mod store;

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

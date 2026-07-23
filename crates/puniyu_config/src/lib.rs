mod error;
pub use error::Error;

pub mod app;
pub mod bot;
pub mod common;
pub mod friend;
pub mod group;

mod types;
#[doc(inline)]
pub use types::*;

use std::path::PathBuf;

pub trait Config: Send + Sync {
	/// 配置名称
	fn name(&self) -> &str;

	/// 配置文件路径
	fn path(&self) -> PathBuf;

	/// 转换为 toml::Value
	fn to_value(&self) -> toml::Value;
}

#[derive(Clone)]
pub struct Entry {
	pub path: PathBuf,
	pub value: toml::Value,
}

impl<T: Config> From<T> for Entry {
	fn from(value: T) -> Self {
		Self {
			path: value.path(),
			value: value.to_value(),
		}
	}
}

pub type ConfigRegistry = puniyu_registry::Registry<Entry>;

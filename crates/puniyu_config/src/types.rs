mod server;
#[doc(inline)]
pub use server::*;
mod adapter;
#[doc(inline)]
pub use adapter::*;
mod plugin;
#[doc(inline)]
pub use plugin::*;
mod list;
#[doc(inline)]
pub use list::*;
mod logger;
#[doc(inline)]
pub use logger::*;
mod command;
#[doc(inline)]
pub use command::*;
mod option;
#[doc(inline)]
pub use option::*;

use smol_str::SmolStr;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigId {
	/// 通过索引标识
	Index(u64),
	/// 通过路径标识
	Path(PathBuf),
}

impl From<u64> for ConfigId {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl From<PathBuf> for ConfigId {
	fn from(path: PathBuf) -> Self {
		Self::Path(path)
	}
}

impl From<&Path> for ConfigId {
	fn from(path: &Path) -> Self {
		Self::Path(path.to_path_buf())
	}
}

impl From<&str> for ConfigId {
	fn from(path: &str) -> Self {
		Self::Path(PathBuf::from(path))
	}
}

#[derive(Debug, Clone)]
pub struct StoredEntry<T> {
	pub name: SmolStr,
	pub path: PathBuf,
	pub value: T,
}

impl<T> PartialEq for StoredEntry<T>
where
	T: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name && self.path == other.path
	}
}
impl<T> Eq for StoredEntry<T> where T: Eq {}

pub trait Config: Send + Sync {
	fn name(&self) -> &str;
	fn path(&self) -> PathBuf;
	fn value(&self) -> toml::Value;
}

mod runtime;
pub use runtime::ProcessRuntime;
mod metadata;
pub use metadata::*;
mod process;
pub use process::IpcProcess;

use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum IpcPluginId<'p> {
	Index(u64),
	Name(Cow<'p, str>),
}

impl From<u64> for IpcPluginId<'_> {
	fn from(value: u64) -> Self {
		Self::Index(value)
	}
}

impl<'p> From<&'p str> for IpcPluginId<'p> {
	fn from(value: &'p str) -> Self {
		Self::Name(Cow::Borrowed(value))
	}
}

impl From<String> for IpcPluginId<'_> {
	fn from(value: String) -> Self {
		Self::Name(Cow::Owned(value))
	}
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ProcessState {
	Starting,
	Running,
	Stopping,
	#[default]
	Stopped,
	Failed,
}

mod bot;
pub(crate) use bot::ConsoleBotRuntime;
mod adapter;
pub use adapter::ConsoleAdapterRuntime;

use bytes::Bytes;
use puniyu_adapter::{path::resource_dir, runtime::AdapterRuntime};
use std::sync::{Arc, LazyLock};

pub(crate) static AVATAR: LazyLock<Bytes> = LazyLock::new(|| {
	let logo_path = resource_dir().join("logo.png");
	std::fs::read(logo_path).unwrap_or_default().into()
});

pub(crate) fn runtime() -> Arc<dyn AdapterRuntime> {
	Arc::new(ConsoleAdapterRuntime::new())
}

mod app;
mod common;
mod logger;
#[doc(inline)]
pub use app::App;
use std::sync::LazyLock;

pub static VERSION: LazyLock<puniyu_version::Version> = LazyLock::new(|| puniyu_version::Version {
	major: const_str::parse!(env!("CARGO_PKG_VERSION_MAJOR"), u64),
	minor: const_str::parse!(env!("CARGO_PKG_VERSION_MINOR"), u64),
	patch: const_str::parse!(env!("CARGO_PKG_VERSION_PATCH"), u64),
});
pub use puniyu_api::bot::*;


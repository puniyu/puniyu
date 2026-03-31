mod app;
mod common;
mod logger;
#[doc(inline)]
pub use app::App;

pub const VERSION: puniyu_version::Version = puniyu_version::Version {
	major: const_str::parse!(env!("CARGO_PKG_VERSION_MAJOR"), u64),
	minor: const_str::parse!(env!("CARGO_PKG_VERSION_MINOR"), u64),
	patch: const_str::parse!(env!("CARGO_PKG_VERSION_PATCH"), u64),
};
pub use puniyu_api::bot::*;

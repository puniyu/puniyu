pub mod adapter;
pub mod bot;
pub mod common;
pub mod contact;
pub mod element;
pub mod event;
pub mod handler;
pub mod macros;
pub mod prelude;
pub mod sender;
pub mod server;

pub use puniyu_logger as logger;
pub use puniyu_types::adapter::{Error, Result};
pub use puniyu_types::version::Version;

#[doc(hidden)]
pub mod private {
	pub use async_trait::async_trait;
	pub use inventory;
	pub use puniyu_common::path::ADAPTER_CONFIG_DIR;
	pub use puniyu_config::ConfigRegistry;
	pub use puniyu_types::adapter::{
		AdapterApi, AdapterBuilder, FriendApi, GroupApi, MessageApi, Result,
	};
	pub use puniyu_types::config::Config;
	pub use puniyu_types::hook::{
		HookBuilder, HookEventType, HookMessageType, HookNotionType, HookRequestType,
		HookType, StatusType,
	};
	pub use puniyu_types::server::ServerType;
	pub use puniyu_types::version::Version;
	pub use toml;
}

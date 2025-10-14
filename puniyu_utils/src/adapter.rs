mod account;
pub use account::AccountInfo;
mod builder;
pub use builder::{AdapterApi, AdapterBase, AvatarSize};
mod info;
pub use info::{
	AdapterCommunication, AdapterInfo, AdapterPlatform, AdapterProtocol, AdapterStandard,
};
mod types;
pub use types::*;

#[derive(Clone)]
pub struct Adapter {
	pub index: u64,
	pub info: AdapterInfo,
	pub api: &'static dyn AdapterApi,
}

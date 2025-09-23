mod account;
pub use account::AccountInfo;
mod builder;
mod info;
pub use builder::{AdapterApi, AdapterBase, AvatarSize};
mod types;
pub use types::*;

pub use info::{
	AdapterCommunication, AdapterInfo, AdapterPlatform, AdapterProtocol, AdapterStandard,
};

#[derive(Clone)]
pub struct Adapter {
	pub index: u64,

	pub adapter: AdapterInfo,

	pub account: AccountInfo,
	pub api: &'static dyn AdapterApi,
	pub self_id: &'static str,
	pub self_name: &'static str,
}

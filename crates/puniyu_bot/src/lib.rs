mod macros;
mod registry;
use std::fmt::Debug;

pub use registry::BotRegistry;
mod types;
use bytes::Bytes;
use puniyu_adapter_api::AdapterApi;
#[doc(inline)]
pub use types::*;

pub trait Bot: Send + Sync + AdapterApi {
	/// 机器人ID
	fn id(&self) -> &str;
	/// 机器人名称
	fn name(&self) -> &str;
	/// 机器人头像
	fn avatar(&self) -> Bytes;
}

impl Debug for dyn Bot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Bot")
			.field("id", &self.id())
			.field("name", &self.name())
			.field("avatar", &self.avatar())
			.finish()
	}
}

impl PartialEq for dyn Bot {
	fn eq(&self, other: &Self) -> bool {
		self.id() == other.id()
	}
}

impl Eq for dyn Bot {}

use crate::account::AccountInfo;
use crate::adapter::{AdapterApi, AdapterInfo, Plugin};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(Clone)]
pub struct Bot {
	/// 适配器信息
	adapter: Arc<dyn Plugin>,
	/// 账户信息
	account: AccountInfo,
}

impl Debug for Bot {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("Bot").field(&self.adapter.info()).field(&self.account).finish()
	}
}
impl PartialEq for Bot {
	fn eq(&self, other: &Self) -> bool {
		self.adapter.info() == other.adapter.info()
	}
}

impl Eq for Bot {}

impl Bot {
	pub fn new<A: Plugin + 'static>(adapter: A, account: AccountInfo) -> Self {
		Self { adapter: Arc::from(adapter), account }
	}
	/// 适配器信息
	pub fn adapter(&self) -> AdapterInfo {
		self.adapter.info()
	}

	/// 适配器Api
	pub fn api(&self) -> AdapterApi {
		self.adapter.api()
	}

	/// Bot自身账号信息
	pub fn account(&self) -> AccountInfo {
		self.account.clone()
	}
}

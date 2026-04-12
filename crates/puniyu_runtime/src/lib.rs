//! # puniyu_runtime
//!
//! 统一的 puniyu 运行时 trait 定义。
//!
//! ## 特性
//!
//! - 提供 [`Runtime`] 作为只读运行时访问视图
//! - 提供 [`AdapterProvider`] 访问适配器信息
//! - 提供 [`AdapterRuntime`] 作为适配器级运行时
//! - 提供 [`AccountProvider`] 访问 bot 账号信息
//! - 提供 [`BotRuntime`] 作为 bot 级运行时
//! - 提供 [`SendMessage`] 作为发送能力 trait
//! - 提供 [`Runtime::downcast_ref`] 访问适配器私有运行时能力

use std::any::Any;

use async_trait::async_trait;
use puniyu_account::AccountInfo;
use puniyu_adapter_types::{AdapterInfo, SendMsgType};
use puniyu_contact::ContactType;
use puniyu_error::Result;
use puniyu_message::Message;

pub trait Runtime: Any + Send + Sync {}

impl<T> Runtime for T where T: Any + Send + Sync {}

impl dyn Runtime {
	pub fn as_any(&self) -> &dyn Any {
		self
	}

	pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
		self.as_any().downcast_ref::<T>()
	}
}

pub trait AdapterProvider: Send + Sync {
	fn adapter_info(&self) -> &AdapterInfo;
}

pub trait AdapterRuntime: Runtime + AdapterProvider + SendMessage {}

impl<T> AdapterRuntime for T where T: Runtime + AdapterProvider + SendMessage {}

pub trait AccountProvider: Send + Sync {
	fn account_info(&self) -> &AccountInfo;
}

#[async_trait]
pub trait SendMessage: Send + Sync {
	async fn send_message(
		&self,
		contact: &ContactType<'_>,
		message: &Message,
	) -> Result<SendMsgType>;
}

pub trait BotRuntime: AdapterRuntime + AccountProvider {}

impl<T> BotRuntime for T where T: AdapterRuntime + AccountProvider {}

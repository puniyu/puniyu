mod macros;
mod registry;
use std::{fmt::Debug, sync::Arc};

use puniyu_error::AnyError;
pub use registry::BotRegistry;
mod types;
use async_trait::async_trait;
use bytes::Bytes;
use puniyu_contact::ContactType;
use puniyu_message::Message;
#[doc(inline)]
pub use types::*;

/// Bot trait — 负责身份信息和消息发送。
///
/// Bot 通过组合持有 Adapter 弱引用，将平台 API 调用委托给适配器。
#[async_trait]
pub trait Bot: Send + Sync {

	fn id(&self) -> &str;

	/// Bot 名称
	fn name(&self) -> &str;

	/// Bot 头像
	fn avatar(&self) -> Bytes;

	/// Bot 连接信息
	fn connection_info(&self) -> &ConnectionInfo;

	/// 适配器信息
	fn adapter_info(&self) -> &AdapterInfo;

	/// 发送消息到指定联系人
	async fn send_message(
		&self,
		contact: &ContactType,
		message: &Message
	) -> AnyError<SendMsgResult>;
}

impl Debug for dyn Bot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Bot")
			.field("id", &self.id())
			.field("name", &self.name())
			.field("connection_info", &self.connection_info())
			.finish()
	}
}

impl PartialEq for dyn Bot {
	fn eq(&self, other: &Self) -> bool {
		self.id() == other.id()
	}
}

impl Eq for dyn Bot {}

#[async_trait]
impl<B: Bot + ?Sized> Bot for Box<B> {
	fn id(&self) -> &str {
		(**self).id()
	}
	fn name(&self) -> &str {
		(**self).name()
	}
	fn avatar(&self) -> Bytes {
		(**self).avatar()
	}
	fn connection_info(&self) -> &ConnectionInfo {
		(**self).connection_info()
	}
	fn adapter_info(&self) -> &AdapterInfo {
		(**self).adapter_info()
	}
	async fn send_message(
		&self,
		contact: &ContactType,
		message: &Message
	) -> AnyError<SendMsgResult> {
		(**self).send_message(contact, message).await
	}
}

#[async_trait]
impl<B: Bot + ?Sized> Bot for Arc<B> {
	fn id(&self) -> &str {
		(**self).id()
	}
	fn name(&self) -> &str {
		(**self).name()
	}
	fn avatar(&self) -> Bytes {
		(**self).avatar()
	}
	fn connection_info(&self) -> &ConnectionInfo {
		(**self).connection_info()
	}
	fn adapter_info(&self) -> &AdapterInfo {
		(**self).adapter_info()
	}
	async fn send_message(
		&self,
		contact: &ContactType,
		message: &Message
	) -> AnyError<SendMsgResult> {
		(**self).send_message(contact, message).await
	}
}
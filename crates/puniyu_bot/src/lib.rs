use std::{fmt::Debug, sync::Arc};

mod types;
#[doc(inline)]
pub use types::*;

use async_trait::async_trait;
use bytes::Bytes;
use puniyu_contact::ContactType;
use puniyu_error::AnyError;
use puniyu_message::Message;

/// Bot trait — 身份信息 + 元信息 + 平台操作。
#[async_trait]
pub trait Bot: Send + Sync {
	/// Bot 唯一标识
	fn id(&self) -> &str;

	/// Bot 名称
	fn name(&self) -> &str;

	/// Bot 头像
	fn avatar(&self) -> Bytes;

	/// 适配器元信息
	fn adapter_info(&self) -> &AdapterInfo;

	/// 连接信息
	fn connection_info(&self) -> &ConnectionInfo;

	/// 发送消息到指定联系人
	async fn send_message(
		&self,
		contact: &ContactType,
		message: &Message,
	) -> AnyError<SendMsgResult>;

	/// 调用平台原生 API
	async fn call_api(
		&self,
		action: &str,
		params: serde_json::Value,
	) -> AnyError<serde_json::Value>;
}

impl Debug for dyn Bot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Bot")
			.field("id", &self.id())
			.field("name", &self.name())
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
impl<B: Bot + ?Sized> Bot for &B {
	fn id(&self) -> &str { (**self).id() }
	fn name(&self) -> &str { (**self).name() }
	fn avatar(&self) -> Bytes { (**self).avatar() }
	fn adapter_info(&self) -> &AdapterInfo { (**self).adapter_info() }
	fn connection_info(&self) -> &ConnectionInfo { (**self).connection_info() }

	async fn send_message(
		&self, contact: &ContactType, message: &Message,
	) -> AnyError<SendMsgResult> {
		(**self).send_message(contact, message).await
	}

	async fn call_api(
		&self, action: &str, params: serde_json::Value,
	) -> AnyError<serde_json::Value> {
		(**self).call_api(action, params).await
	}
}

#[async_trait]
impl<B: Bot + ?Sized> Bot for Box<B> {
	fn id(&self) -> &str { (**self).id() }
	fn name(&self) -> &str { (**self).name() }
	fn avatar(&self) -> Bytes { (**self).avatar() }
	fn adapter_info(&self) -> &AdapterInfo { (**self).adapter_info() }
	fn connection_info(&self) -> &ConnectionInfo { (**self).connection_info() }

	async fn send_message(
		&self, contact: &ContactType, message: &Message,
	) -> AnyError<SendMsgResult> {
		(**self).send_message(contact, message).await
	}

	async fn call_api(
		&self, action: &str, params: serde_json::Value,
	) -> AnyError<serde_json::Value> {
		(**self).call_api(action, params).await
	}
}

#[async_trait]
impl<B: Bot + ?Sized> Bot for Arc<B> {
	fn id(&self) -> &str { (**self).id() }
	fn name(&self) -> &str { (**self).name() }
	fn avatar(&self) -> Bytes { (**self).avatar() }
	fn adapter_info(&self) -> &AdapterInfo { (**self).adapter_info() }
	fn connection_info(&self) -> &ConnectionInfo { (**self).connection_info() }

	async fn send_message(
		&self, contact: &ContactType, message: &Message,
	) -> AnyError<SendMsgResult> {
		(**self).send_message(contact, message).await
	}

	async fn call_api(
		&self, action: &str, params: serde_json::Value,
	) -> AnyError<serde_json::Value> {
		(**self).call_api(action, params).await
	}
}

use std::{fmt::Debug, sync::Arc};

mod types;
#[doc(inline)]
pub use types::*;

use async_trait::async_trait;
use puniyu_contact::ContactType;
use puniyu_element::File;
use puniyu_error::AnyError;
use puniyu_message::Message;


#[async_trait]
pub trait Bot: Send + Sync {
	/// Bot 唯一标识
	fn self_id(&self) -> &str;

	/// Bot 名称
	fn name(&self) -> &str;

	/// Bot 头像
	fn avatar(&self) -> File;

	/// 适配器元信息
	fn adapter_info(&self) -> &AdapterInfo;

	/// 连接信息
	fn connection_info(&self) -> &ConnectionInfo;

		/// 调用平台原生 API
	async fn call_api(
		&self,
		action: &str,
		params: serde_json::Value,
	) -> AnyError<serde_json::Value>;

	/// 发送消息到指定联系人
	async fn send_message(
		&self,
		contact: &ContactType,
		message: &Message,
	) -> AnyError<SendMsgResult> {
		let params = serde_json::json!({
			"contact": contact,
			"message": message,
		});
		let result = self.call_api("send_message", params).await?;
		serde_json::from_value(result).map_err(Into::into)
	}
}

impl Debug for dyn Bot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Bot")
			.field("id", &self.self_id())
			.field("name", &self.name())
			.finish()
	}
}

impl PartialEq for dyn Bot {
	fn eq(&self, other: &Self) -> bool {
		self.self_id() == other.self_id()
	}
}

impl Eq for dyn Bot {}

macro_rules! impl_bot_deref {
	($wrapper:ty) => {
		#[async_trait]
		impl<B: Bot + ?Sized> Bot for $wrapper {
			fn self_id(&self) -> &str { (**self).self_id() }
			fn name(&self) -> &str { (**self).name() }
			fn avatar(&self) -> File { (**self).avatar() }
			fn adapter_info(&self) -> &AdapterInfo { (**self).adapter_info() }
			fn connection_info(&self) -> &ConnectionInfo { (**self).connection_info() }

			async fn call_api(
				&self, action: &str, params: serde_json::Value,
			) -> AnyError<serde_json::Value> {
				(**self).call_api(action, params).await
			}

			async fn send_message(
				&self, contact: &ContactType, message: &Message,
			) -> AnyError<SendMsgResult> {
				(**self).send_message(contact, message).await
			}
		}
	};
}

impl_bot_deref!(&B);
impl_bot_deref!(Box<B>);
impl_bot_deref!(Arc<B>);

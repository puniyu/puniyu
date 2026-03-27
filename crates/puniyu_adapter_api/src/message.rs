use super::inner::Error;
use async_trait::async_trait;
use puniyu_adapter_types::{MessageInfo, MessageType, SendMsgType};
use puniyu_contact::ContactType;
use puniyu_error::Result;
use puniyu_message::Message;

/// 消息操作接口。
#[async_trait]
pub trait MessageApi: Send + Sync {
	/// 发送消息。
	async fn send_msg(&self, contact: &ContactType, message: &Message) -> Result<SendMsgType> {
		Err(Error::NotImpl.into())
	}

	/// 撤回消息。
	async fn recall_msg(&self, message_id: &str) -> Result<()> {
		Err(Error::NotImpl.into())
	}

	/// 获取消息。
	async fn get_msg(&self, message_id: &str) -> Result<MessageType> {
		Err(Error::NotImpl.into())
	}

	/// 获取历史消息。
	async fn get_history_msg(
		&self,
		contact: &ContactType,
		message: &MessageType,
		count: u8,
	) -> Result<Vec<MessageInfo>> {
		Err(Error::NotImpl.into())
	}
}

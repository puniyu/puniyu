use super::{Error, MessageInfo, MessageType, Result, SendMsgType};
use crate::contact::ContactType;
use crate::element;
use async_trait::async_trait;

#[async_trait]
pub trait MessageApi: Send + Sync {
	/// 发送消息
	///
	/// ## 参数
	/// `_contact` - 联系人
	/// `_message` - 消息元素
	///
	async fn send_msg(
		&self,
		_contact: ContactType,
		_message: element::Message,
	) -> Result<SendMsgType> {
		Err(Error::NotImpl)
	}

	/// 撤回消息
	///
	/// ## 参数
	/// `_message_id` - 消息ID
	///
	async fn recall_msg(&self, _message_id: &str) -> Result<()> {
		Err(Error::NotImpl)
	}

	/// 获取消息
	///
	/// ## 参数
	/// `_message_id` - 消息ID
	///
	async fn get_msg(&self, _message_id: &str) -> Result<MessageType> {
		Err(Error::NotImpl)
	}

	/// 获取历史消息
	///     指定消息位置开始向前获取历史消息，按时间倒序排列
	///
	/// ## 参数
	/// `_contact` - 联系人
	/// `_message` - 消息ID获取消息序号
	/// `_count` - 获取消息数量，最大值为20
	///
	async fn get_history_msg(
		&self,
		_contact: ContactType,
		_message: MessageType,
		_count: u8,
	) -> Result<Vec<MessageInfo>> {
		Err(Error::NotImpl)
	}
}

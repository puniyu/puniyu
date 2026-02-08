use super::inner::Error;
use crate::adapter::{MessageInfo, MessageType, SendMsgType};
use crate::contact::ContactType;
use crate::element;
use crate::handler::HandlerResult;
use async_trait::async_trait;

#[async_trait]
pub trait MessageApi: Send + Sync {
	/// 发送消息
	///
	/// ## 参数
	/// `contact` - 联系人
	/// `message` - 消息元素
	///
	async fn send_msg<'m>(
		&self,
		contact: ContactType,
		message: element::Message<'m>,
	) -> HandlerResult<SendMsgType> {
		Err(Error::NotImpl.into())
	}

	/// 撤回消息
	///
	/// ## 参数
	/// `message_id` - 消息ID
	///
	async fn recall_msg(&self, message_id: &str) -> HandlerResult<()> {
		Err(Error::NotImpl.into())
	}

	/// 获取消息
	///
	/// ## 参数
	/// `message_id` - 消息ID
	///
	async fn get_msg(&self, message_id: &str) -> HandlerResult<MessageType> {
		Err(Error::NotImpl.into())
	}

	/// 获取历史消息
	///     指定消息位置开始向前获取历史消息，按时间倒序排列
	///
	/// ## 参数
	/// `contact` - 联系人
	/// `message` - 消息ID获取消息序号
	/// `count` - 获取消息数量，最大值为20
	///
	async fn get_history_msg(
		&self,
		contact: ContactType,
		message: MessageType,
		count: u8,
	) -> HandlerResult<Vec<MessageInfo>> {
		Err(Error::NotImpl.into())
	}
}

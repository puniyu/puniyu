use super::inner::Error;
use async_trait::async_trait;
use puniyu_adapter_types::{MessageInfo, MessageType, SendMsgType};
use puniyu_contact::ContactType;
use puniyu_error::Result;
use puniyu_message::Message;

/// 消息 API trait
///
/// 提供消息相关的操作接口，包括发送、撤回、获取消息等功能。
///
/// # 示例
///
/// ## 实现 MessageApi
///
/// ```rust,ignore
/// use puniyu_adapter_core::api::MessageApi;
/// use puniyu_contact::ContactType;
/// use puniyu_message::Message;
/// use puniyu_error::Result;
/// use async_trait::async_trait;
///
/// struct MyMessageApi;
///
/// #[async_trait]
/// impl MessageApi for MyMessageApi {
///     async fn send_msg(&self, contact: &ContactType, message: &Message) -> Result<SendMsgType> {
///         // 实现发送消息逻辑
///         Ok(SendMsgType {
///             message_id: "12345".to_string(),
///             time: 1234567890,
///         })
///     }
/// }
/// ```
///
/// ## 使用 MessageApi
///
/// ```rust,ignore
/// use puniyu_adapter_core::api::MessageApi;
/// use puniyu_contact::Contact;
/// use puniyu_message::Message;
///
/// async fn send_text_message(api: &dyn MessageApi) {
///     let contact = Contact::friend("123456");
///     let message = Message::from("Hello, World!");
///     
///     match api.send_msg(&contact, &message).await {
///         Ok(info) => println!("消息已发送，ID: {}", info.message_id),
///         Err(e) => eprintln!("发送失败: {}", e),
///     }
/// }
/// ```
#[async_trait]
pub trait MessageApi: Send + Sync {
	/// 发送消息
	///
	/// 向指定联系人发送消息。
	///
	/// # 参数
	///
	/// - `contact` - 联系人（好友或群组）
	/// - `message` - 消息内容
	///
	/// # 返回值
	///
	/// 返回 `SendMsgType`，包含消息 ID 和发送时间
	///
	/// # 错误
	///
	/// 如果发送失败，返回错误
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_contact::Contact;
	/// use puniyu_message::Message;
	///
	/// let contact = Contact::friend("123456");
	/// let message = Message::from("Hello!");
	/// let result = api.send_msg(&contact, &message).await?;
	/// println!("消息 ID: {}", result.message_id);
	/// ```
	async fn send_msg(&self, contact: &ContactType, message: &Message) -> Result<SendMsgType> {
		Err(Error::NotImpl.into())
	}

	/// 撤回消息
	///
	/// 撤回已发送的消息。
	///
	/// # 参数
	///
	/// - `message_id` - 要撤回的消息 ID
	///
	/// # 错误
	///
	/// 如果撤回失败（如消息不存在、超时等），返回错误
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// api.recall_msg("12345").await?;
	/// println!("消息已撤回");
	/// ```
	async fn recall_msg(&self, message_id: &str) -> Result<()> {
		Err(Error::NotImpl.into())
	}

	/// 获取消息
	///
	/// 根据消息 ID 获取消息详情。
	///
	/// # 参数
	///
	/// - `message_id` - 消息 ID
	///
	/// # 返回值
	///
	/// 返回 `MessageType`，包含消息的标识信息
	///
	/// # 错误
	///
	/// 如果消息不存在或获取失败，返回错误
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let message = api.get_msg("12345").await?;
	/// println!("消息序号: {:?}", message.message_seq);
	/// ```
	async fn get_msg(&self, message_id: &str) -> Result<MessageType> {
		Err(Error::NotImpl.into())
	}

	/// 获取历史消息
	///
	/// 从指定消息位置开始向前获取历史消息，按时间倒序排列。
	///
	/// # 参数
	///
	/// - `contact` - 联系人（好友或群组）
	/// - `message` - 起始消息位置（消息 ID 或序号）
	/// - `count` - 获取消息数量，最大值为 20
	///
	/// # 返回值
	///
	/// 返回消息信息列表，按时间倒序排列
	///
	/// # 错误
	///
	/// 如果获取失败，返回错误
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_contact::Contact;
	/// use puniyu_adapter_core::types::MessageType;
	///
	/// let contact = Contact::group("123456");
	/// let start_msg = MessageType::from_id("12345");
	/// let messages = api.get_history_msg(&contact, &start_msg, 10).await?;
	///
	/// for msg in messages {
	///     println!("消息: {}", msg.message_id);
	/// }
	/// ```
	async fn get_history_msg(
		&self,
		contact: &ContactType,
		message: &MessageType,
		count: u8,
	) -> Result<Vec<MessageInfo>> {
		Err(Error::NotImpl.into())
	}
}

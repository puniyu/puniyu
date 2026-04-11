use puniyu_account::AccountInfo;
use puniyu_runtime::Runtime;
use puniyu_adapter_types::{AdapterInfo, SendMsgType};
use puniyu_bot::Bot;
use puniyu_contact::ContactType;
use puniyu_message::Message;

/// 机器人上下文
///
/// 提供对机器人实例的访问，包括运行时和账号信息。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_context::BotContext;
///
/// let bot_context = BotContext::new(&bot);
///
/// // 访问运行时
/// let runtime = bot_context.runtime();
///
/// // 访问账号信息
/// let account = bot_context.account();
/// ```
#[derive(Clone)]
pub struct BotContext<'c> {
	inner: &'c Bot,
}

impl<'c> BotContext<'c> {
	/// 创建新的机器人上下文
	///
	/// # 参数
	///
	/// - `bot` - 机器人实例的引用
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let bot_context = BotContext::new(&bot);
	/// ```
	pub fn new(bot: &'c Bot) -> Self {
		Self { inner: bot }
	}

	/// 获取适配器的引用
	///
	/// 返回适配器的引用。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let adapter = bot_context.adapter();
	/// ```
	pub fn adapter(&self) -> &AdapterInfo {
		self.inner.adapter()
	}

	/// 获取适配器运行时。
	///
	/// 返回适配器运行时的只读视图，可用于访问适配器私有能力。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let runtime = bot_context.runtime();
	///
	/// // 访问适配器私有能力
	/// let concrete = runtime.downcast_ref::<MyRuntime>();
	/// ```
	pub fn runtime(&self) -> &dyn Runtime {
		self.inner.runtime()
	}

	/// 向指定联系人发送消息。
	pub async fn send_message<M>(
		&self,
		contact: &ContactType<'_>,
		message: M,
	) -> puniyu_error::Result<SendMsgType>
	where
		M: Into<Message>,
	{
		let message = message.into();
		self.inner.send_message(contact, &message).await
	}

	/// 获取账号信息
	///
	/// 返回机器人账号信息的引用。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let account = bot_context.account();
	/// ```
	pub fn account(&self) -> &AccountInfo {
		self.inner.account()
	}
}

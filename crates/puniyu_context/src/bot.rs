use puniyu_account::AccountInfo;
use puniyu_adapter_api::AdapterApi;
use puniyu_adapter_types::AdapterInfo;
use puniyu_bot::Bot;

/// 机器人上下文
///
/// 提供对机器人实例的访问，包括 API 和账号信息。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_context::BotContext;
///
/// let bot_context = BotContext::new(&bot);
///
/// // 访问 API
/// let api = bot_context.api();
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

	/// 获取适配器 API
	///
	/// 返回适配器 API 的引用，用于调用各种 API 方法。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let api = bot_context.api();
	/// api.message().send_msg(&contact, &message).await?;
	/// ```
	pub fn api(&self) -> &AdapterApi {
		self.inner.api()
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

use puniyu_account::AccountInfo;
use puniyu_adapter_core::api::AdapterApi;
use puniyu_adapter_core::types::info::AdapterInfo;
use puniyu_bot::Bot;

/// 机器人上下文
///
/// 提供对机器人实例的访问，包括 API 和账号信息。
///
/// 使用 `Arc<dyn Bot>` 实现动态分发，支持任意实现了 `Bot` trait 的类型。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_context::BotContext;
/// use std::sync::Arc;
///
/// let bot_context = BotContext::new(Arc::new(bot));
///
/// // 访问 API
/// let api = bot_context.api();
///
/// // 访问账号信息
/// let account = bot_context.account();
/// ```
#[derive(Clone)]
pub struct BotContext<'c> {
	bot: &'c Bot,
}

impl<'c> BotContext<'c> {
	/// 创建新的机器人上下文
	///
	/// # 参数
	///
	/// - `bot` - 机器人实例的 `Arc` 智能指针
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use std::sync::Arc;
	///
	/// let bot_context = BotContext::new(Arc::new(bot));
	/// ```
	pub fn new(bot: &'c Bot) -> Self {
		Self { bot }
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
		self.bot.adapter()
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
		self.bot.api()
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
		self.bot.account()
	}
}

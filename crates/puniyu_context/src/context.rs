mod sub;
pub use sub::SubContext;
mod path;
pub use path::PathConetxt;
mod bot;
pub use bot::BotContext;
mod action;
pub use action::ActionConetxt;
mod service;
pub use service::ServiceContext;

use std::sync::Arc;

pub struct Context {
	bot: Arc<BotContext>,
	path: Arc<PathConetxt>,
}

impl Context {
	/// 创建根上下文。
	pub fn new(path: puniyu_path::Path) -> Self {
		Self { bot: Arc::new(BotContext::new()), path: Arc::new(PathConetxt::new(path)) }
	}
	/// 创建插件子上下文
	pub fn sub(&self) -> SubContext {
		SubContext {
			bot: self.bot.clone(),
			service: Arc::new(ServiceContext::new()),
			action: Arc::new(ActionConetxt::new()),
			path: self.path.clone(),
		}
	}
}

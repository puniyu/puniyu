mod handler;
pub use handler::CommandHandler;

use puniyu_action::Action;
use puniyu_extractor::Extractor;
use puniyu_handler::Handler;
use puniyu_matcher::Matcher;
use puniyu_matcher_combinator::AndMatcher;
use puniyu_matcher_event::MessageMatcher;
use smol_str::SmolStr;

/// 命令行为。
///
/// 绑定 Matcher + Extractor + Handler
pub struct Command<M: Matcher, H: Handler, E: Extractor> {
	name: SmolStr,
	description: Option<SmolStr>,
	matcher: AndMatcher<MessageMatcher, M>,
	handler: CommandHandler<H, E>,
	priority: u32,
	block: bool,
}

impl<M: Matcher, H: Handler, E: Extractor> Command<M, H, E> {
	/// 创建命令。
	pub fn new(
		name: impl Into<SmolStr>, 
		matcher: M, 
		extractor: E, 
		handler: H
	) -> Self {
		Self {
			name: name.into(),
			description: None,
			matcher: AndMatcher::new(MessageMatcher, matcher),
			handler: CommandHandler::new(handler, extractor),
			priority: 500,
			block: false,
		}
	}
	pub fn description(mut self, description: impl Into<SmolStr>) -> Self {
		self.description = Some(description.into());
		self
	}
	pub fn priority(mut self, priority: u32) -> Self {
		self.priority = priority;
		self
	}

	pub fn block(mut self, block: bool) -> Self {
		self.block = block;
		self
	}
}

impl<M: Matcher, H: Handler, E: Extractor> Action for Command<M, H, E> {
	fn name(&self) -> &str {
		self.name.as_str()
	}

	fn matcher(&self) -> &dyn Matcher {
		&self.matcher
	}

	fn handler(&self) -> &dyn Handler {
		&self.handler
	}

	fn description(&self) -> Option<&str> {
		self.description.as_deref()
	}

	fn priority(&self) -> u32 {
		self.priority
	}

	fn block(&self) -> bool {
		self.block
	}
}

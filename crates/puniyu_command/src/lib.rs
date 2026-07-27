mod matcher;
use bon::Builder;
pub use matcher::CommandMatcher;

use puniyu_action::Action;
use puniyu_handler::Handler;
use puniyu_matcher::Matcher;
use puniyu_matcher_combinator::And;
use smol_str::SmolStr;

#[derive(Builder)]
pub struct Command<M: Matcher, H: Handler> {
	#[builder(into)]
	name: SmolStr,
	matcher: And<CommandMatcher, M>,
	handler: H,
	#[builder(default = 500)]
	priority: u32,
	#[builder(default = false)]
	block: bool,
}

impl<M: Matcher, H: Handler> Command<M, H> {
	pub fn new(
		name: impl Into<SmolStr>,
		matcher: M,
		handler: H,
		priority: u32,
		block: bool,
	) -> Self {
		Self {
			name: name.into(),
			matcher: And::new(CommandMatcher, matcher),
			handler,
			priority,
			block,
		}
	}
}

impl<M: Matcher, H: Handler> Action for Command<M, H> {
	fn name(&self) -> &str {
		&self.name
	}

	fn matcher(&self) -> &dyn Matcher {
		&self.matcher
	}

	fn handler(&self) -> &dyn Handler {
		&self.handler
	}

	fn priority(&self) -> u32 {
		self.priority
	}

	fn block(&self) -> bool {
		self.block
	}
}

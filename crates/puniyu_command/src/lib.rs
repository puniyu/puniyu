use bon::Builder;
use puniyu_action::Action;
use puniyu_handler::Handler;
use puniyu_matcher::Matcher;
use puniyu_matcher_combinator::AndMatcher;
use puniyu_matcher_event::MessageMatcher;
use smol_str::SmolStr;

/// 命令行为
///
///
/// # 示例
///
/// ```ignore
/// let cmd = Command::builder()
///     .name("echo")
///     .matcher(KeyWord::new("echo"))
///     .handler(my_handler)
///     .description("echo back")
///     .priority(100)
///     .build();
/// ```
#[derive(Builder)]
pub struct Command<M: Matcher, H: Handler> {
	#[builder(into)]
	name: SmolStr,
	#[builder(into)]
	description: Option<SmolStr>,
	matcher: AndMatcher<MessageMatcher, M>,
	handler: H,
	#[builder(default = 500)]
	priority: u32,
	#[builder(default = false)]
	block: bool,
}

impl<M: Matcher, H: Handler> Command<M, H> {
	/// 创建命令。
	pub fn new(
		name: impl Into<SmolStr>,
		matcher: M,
		handler: H,
	) -> Self {
		Self::builder()
			.name(name)
			.matcher(AndMatcher::new(MessageMatcher, matcher))
			.handler(handler)
			.build()
	}
}

impl<M: Matcher, H: Handler> Action for Command<M, H> {
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

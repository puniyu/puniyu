use async_trait::async_trait;
use puniyu_matcher::Matcher;
use puniyu_session::EventSession;

/// 组合匹配器：取反。
pub struct NotMatcher<M: Matcher> {
	matcher: M,
}

impl<M: Matcher> NotMatcher<M> {
	pub fn new(matcher: M) -> Self {
		Self { matcher }
	}
}

#[async_trait]
impl<M: Matcher> Matcher for NotMatcher<M> {
	async fn matches(&self, session: &EventSession) -> bool {
		!self.matcher.matches(session).await
	}
}

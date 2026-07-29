use async_trait::async_trait;
use puniyu_matcher::Matcher;
use puniyu_session::EventSession;

/// 组合匹配器：任一子匹配器匹配即成功。
pub struct OrMatcher<A: Matcher, B: Matcher> {
	a: A,
	b: B,
}

impl<A: Matcher, B: Matcher> OrMatcher<A, B> {
	pub fn new(a: A, b: B) -> Self {
		Self { a, b }
	}
}

#[async_trait]
impl<A: Matcher, B: Matcher> Matcher for OrMatcher<A, B> {
	async fn matches(&self, session: &EventSession) -> bool {
		self.a.matches(session).await || self.b.matches(session).await
	}
}

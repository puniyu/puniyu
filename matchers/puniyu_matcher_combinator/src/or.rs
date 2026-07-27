use async_trait::async_trait;
use puniyu_matcher::Matcher;
use puniyu_param::Params;
use puniyu_session::EventSession;

/// 组合匹配器：任一子匹配器匹配即成功。
///
/// 优先返回第一个子匹配器的参数。
pub struct Or<A: Matcher, B: Matcher> {
	a: A,
	b: B,
}

impl<A: Matcher, B: Matcher> Or<A, B> {
	pub fn new(a: A, b: B) -> Self {
		Self { a, b }
	}
}

#[async_trait]
impl<A: Matcher, B: Matcher> Matcher for Or<A, B> {
	async fn matches(&self, session: &EventSession) -> Option<Params> {
		self.a.matches(session).await.or(self.b.matches(session).await)
	}
}

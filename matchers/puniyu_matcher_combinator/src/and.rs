use async_trait::async_trait;
use puniyu_matcher::Matcher;
use puniyu_param::Params;
use puniyu_session::MessageSession;

/// 组合匹配器：两个子匹配器都匹配时才成功。
///
/// 合并两个子匹配器返回的参数。
pub struct And<A: Matcher, B: Matcher> {
	a: A,
	b: B,
}

impl<A: Matcher, B: Matcher> And<A, B> {
	pub fn new(a: A, b: B) -> Self {
		Self { a, b }
	}
}

#[async_trait]
impl<A: Matcher, B: Matcher> Matcher for And<A, B> {
	async fn matches(&self, session: &MessageSession) -> Option<Params> {
		let params_a = self.a.matches(session).await?;
		let params_b = self.b.matches(session).await?;
		let mut merged = Params::new();
		for (k, v) in params_a {
			merged.push(k, v);
		}
		for (k, v) in params_b {
			merged.push(k, v);
		}
		Some(merged)
	}
}

use async_trait::async_trait;
use puniyu_matcher::Matcher;
use puniyu_param::Params;
use puniyu_session::EventSession;

/// 组合匹配器：取反。
///
/// 子匹配器不匹配时返回空参数，匹配时返回 `None`。
pub struct Not<M: Matcher> {
	matcher: M,
}

impl<M: Matcher> Not<M> {
	pub fn new(matcher: M) -> Self {
		Self { matcher }
	}
}

#[async_trait]
impl<M: Matcher> Matcher for Not<M> {
	async fn matches(&self, session: &EventSession) -> Option<Params> {
		match self.matcher.matches(session).await {
			Some(_) => None,
			None => Some(Params::new()),
		}
	}
}

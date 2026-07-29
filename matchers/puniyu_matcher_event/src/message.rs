use async_trait::async_trait;
use puniyu_matcher::Matcher;
use puniyu_session::EventSession;

pub struct MessageMatcher;

#[async_trait]
impl Matcher for MessageMatcher {
	async fn matches(&self, session: &EventSession) -> bool {
		session.as_message().is_some()
	}
}

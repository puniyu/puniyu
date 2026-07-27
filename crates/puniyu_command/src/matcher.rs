use async_trait::async_trait;
use puniyu_matcher::Matcher;
use puniyu_param::Params;
use puniyu_session::EventSession;

pub struct CommandMatcher;

#[async_trait]
impl Matcher for CommandMatcher {
	async fn matches(&self, session: &EventSession) -> Option<Params> {
        session.as_message()?;
        Some(Params::new())
    }
}

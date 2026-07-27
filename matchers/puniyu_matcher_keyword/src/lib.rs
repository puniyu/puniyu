use async_trait::async_trait;
use puniyu_matcher::Matcher;
use puniyu_param::Params;
use puniyu_session::EventSession;
use smol_str::SmolStr;

pub struct KeyWord {
	name: SmolStr,
}

impl KeyWord {
	pub fn new(name: impl Into<SmolStr>) -> Self {
		Self { name: name.into() }
	}
}
#[async_trait]
impl Matcher for KeyWord {
	async fn matches(&self, session: &EventSession) -> Option<Params> {
		session.get_text().contains(&self.name.as_str()).then_some(Params::new())
	}
}

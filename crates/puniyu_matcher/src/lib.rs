use async_trait::async_trait;
use puniyu_param::Params;
use puniyu_session::EventSession;
use std::sync::Arc;

/// 通用匹配器
#[async_trait]
pub trait Matcher: Send + Sync {
	/// 尝试匹配Session
	async fn matches(&self, session: &EventSession) -> Option<Params>;
}

#[async_trait]
impl<M: Matcher + ?Sized> Matcher for Box<M> {
	async fn matches(&self, session: &EventSession) -> Option<Params> {
		(**self).matches(session).await
	}
}

#[async_trait]
impl<M: Matcher + ?Sized> Matcher for &M {
	async fn matches(&self, session: &EventSession) -> Option<Params> {
		(**self).matches(session).await
	}
}

#[async_trait]
impl<M: Matcher + ?Sized> Matcher for Arc<M> {
	async fn matches(&self, session: &EventSession) -> Option<Params> {
		(**self).matches(session).await
	}
}
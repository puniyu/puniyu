use async_trait::async_trait;
use puniyu_param::Params;
use puniyu_session::MessageSession;
use std::sync::Arc;

/// 通用匹配器
#[async_trait]
pub trait Matcher: Send + Sync {
	/// 尝试匹配消息
	async fn matches(&self, session: &MessageSession) -> Option<Params>;
}

#[async_trait]
impl<M: Matcher + ?Sized> Matcher for Box<M> {
	async fn matches(&self, session: &MessageSession) -> Option<Params> {
		(**self).matches(session).await
	}
}

#[async_trait]
impl<M: Matcher + ?Sized> Matcher for Arc<M> {
	async fn matches(&self, session: &MessageSession) -> Option<Params> {
		(**self).matches(session).await
	}
}
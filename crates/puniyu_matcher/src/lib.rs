use async_trait::async_trait;
use puniyu_session::EventSession;
use std::sync::Arc;

/// 通用匹配器。
///
/// 只负责判断事件是否匹配，不产出数据。
#[async_trait]
pub trait Matcher: Send + Sync {
	/// 尝试匹配 Session，返回是否匹配。
	async fn matches(&self, session: &EventSession) -> bool;
}

#[async_trait]
impl<M: Matcher + ?Sized> Matcher for Box<M> {
	async fn matches(&self, session: &EventSession) -> bool {
		(**self).matches(session).await
	}
}

#[async_trait]
impl<M: Matcher + ?Sized> Matcher for &M {
	async fn matches(&self, session: &EventSession) -> bool {
		(**self).matches(session).await
	}
}

#[async_trait]
impl<M: Matcher + ?Sized> Matcher for Arc<M> {
	async fn matches(&self, session: &EventSession) -> bool {
		(**self).matches(session).await
	}
}

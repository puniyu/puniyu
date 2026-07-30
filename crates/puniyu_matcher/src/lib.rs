use async_trait::async_trait;
use puniyu_session::EventSession;
use std::sync::Arc;

/// 通用匹配器。
///
/// 只负责判断事件是否匹配，不产出数据
#[async_trait]
pub trait Matcher: Send + Sync {
	/// 尝试匹配 Session，返回是否匹配。
	async fn matches(&self, session: &EventSession) -> bool;
}

macro_rules! impl_matcher_deref {
	($wrapper:ty) => {
		#[async_trait]
		impl<M: Matcher + ?Sized> Matcher for $wrapper {
			async fn matches(&self, session: &EventSession) -> bool {
				(**self).matches(session).await
			}
		}
	};
}

impl_matcher_deref!(&M);
impl_matcher_deref!(Box<M>);
impl_matcher_deref!(Arc<M>);

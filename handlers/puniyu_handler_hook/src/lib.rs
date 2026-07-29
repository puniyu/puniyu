mod before;
pub use before::BeforeHandler;
mod after;
pub use after::AfterHandler;

use async_trait::async_trait;
use puniyu_handler::Handler;

/// 带钩子的处理器。
pub struct HookHandler<B: Handler, H: Handler, A: Handler> {
	before: Option<BeforeHandler<B>>,
	handler: H,
	after: Option<AfterHandler<A>>,
}

impl<H: Handler> HookHandler<H, H, H> {
	pub fn new(handler: H) -> Self {
		Self { before: None, handler, after: None }
	}
}

impl<B: Handler, H: Handler, A: Handler> HookHandler<B, H, A> {
	pub fn before<B2: Handler>(self, before: B2) -> HookHandler<B2, H, A> {
		HookHandler { before: Some(BeforeHandler::new(before)), handler: self.handler, after: self.after }
	}

	pub fn after<A2: Handler>(self, after: A2) -> HookHandler<B, H, A2> {
		HookHandler { before: self.before, handler: self.handler, after: Some(AfterHandler::new(after)) }
	}
}

#[async_trait]
impl<B: Handler, H: Handler, A: Handler> Handler for HookHandler<B, H, A> {
	async fn handle(
		&self,
		session: puniyu_session::EventSession,
	) -> puniyu_error::AnyError {
		if let Some(before) = &self.before {
			before.handle(session.clone()).await?;
		}
		let result = self.handler.handle(session.clone()).await;
		if let Some(after) = &self.after {
			after.handle(session).await.ok();
		}
		result
	}
}

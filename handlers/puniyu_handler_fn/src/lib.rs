use std::future::Future;
use std::pin::Pin;

use async_trait::async_trait;
use puniyu_error::AnyError;
use puniyu_handler::Handler;
use puniyu_session::EventSession;

type BoxFuture = Pin<Box<dyn Future<Output = AnyError> + Send>>;

type HandlerFn = Box<dyn Fn(EventSession) -> BoxFuture + Send + Sync>;

pub struct FnHandler {
	f: HandlerFn,
}

impl FnHandler {
	pub fn new<F, Fut>(f: F) -> Self
	where
		F: Fn(EventSession) -> Fut + Send + Sync + 'static,
		Fut: Future<Output = AnyError> + Send + 'static,
	{
		Self { f: Box::new(move |s| Box::pin(f(s))) }
	}
}

#[async_trait]
impl Handler for FnHandler {
	async fn handle(&self, session: EventSession) -> AnyError {
		(self.f)(session).await
	}
}

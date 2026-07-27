use std::future::Future;
use std::pin::Pin;

use async_trait::async_trait;
use puniyu_error::AnyError;
use puniyu_handler::Handler;
use puniyu_param::Params;
use puniyu_session::MessageSession;

type BoxFuture = Pin<Box<dyn Future<Output = AnyError> + Send>>;

type HandlerFn = Box<dyn Fn(MessageSession, Params) -> BoxFuture + Send + Sync>;

pub struct FnHandler {
	f: HandlerFn,
}

impl FnHandler {
	pub fn new<F, Fut>(f: F) -> Self
	where
		F: Fn(MessageSession, Params) -> Fut + Send + Sync + 'static,
		Fut: Future<Output = AnyError> + Send + 'static,
	{
		Self { f: Box::new(move |s, p| Box::pin(f(s, p))) }
	}
}

#[async_trait]
impl Handler for FnHandler {
	async fn handle(&self, session: MessageSession, params: Params) -> AnyError {
		(self.f)(session, params).await
	}
}

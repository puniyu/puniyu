use std::future::Future;
use std::pin::Pin;

use async_trait::async_trait;
use puniyu_error::AnyError;
use puniyu_handler::Handler;
use puniyu_param::ParamValue;
use puniyu_session::EventSession;

type BoxFuture = Pin<Box<dyn Future<Output = AnyError> + Send>>;

type HandlerFn = Box<dyn Fn(EventSession, ParamValue) -> BoxFuture + Send + Sync>;

pub struct FnHandler {
	f: HandlerFn,
}

impl FnHandler {
	pub fn new<F, Fut>(f: F) -> Self
	where
		F: Fn(EventSession, ParamValue) -> Fut + Send + Sync + 'static,
		Fut: Future<Output = AnyError> + Send + 'static,
	{
		Self { f: Box::new(move |s, p| Box::pin(f(s, p))) }
	}
}

#[async_trait]
impl Handler for FnHandler {
	async fn handle(&self, session: EventSession, params: ParamValue) -> AnyError {
		(self.f)(session, params).await
	}
}

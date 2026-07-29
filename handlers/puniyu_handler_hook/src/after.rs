use async_trait::async_trait;
use puniyu_handler::Handler;
use puniyu_param::ParamValue;

pub struct AfterHandler<H: Handler> {
	handler: H,
}

impl<H: Handler> AfterHandler<H> {
	pub fn new(handler: H) -> Self {
		Self { handler }
	}
}

#[async_trait]
impl<H: Handler> Handler for AfterHandler<H> {
	async fn handle(
		&self,
		session: puniyu_session::EventSession,
		params: ParamValue,
	) -> puniyu_error::AnyError {
		self.handler.handle(session, params).await.ok();
		Ok(())
	}
}

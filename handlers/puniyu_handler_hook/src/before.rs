use async_trait::async_trait;
use puniyu_handler::Handler;
use puniyu_param::ParamValue;

pub struct BeforeHandler<H: Handler> {
	handler: H,
}

impl<H: Handler> BeforeHandler<H> {
	pub fn new(handler: H) -> Self {
		Self { handler }
	}
}

#[async_trait]
impl<H: Handler> Handler for BeforeHandler<H> {
	async fn handle(
		&self,
		session: puniyu_session::EventSession,
		params: ParamValue,
	) -> puniyu_error::AnyError {
		self.handler.handle(session, params).await
	}
}

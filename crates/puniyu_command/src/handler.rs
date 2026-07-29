use async_trait::async_trait;
use puniyu_extractor::Extractor;
use puniyu_handler::Handler;
use puniyu_param::ParamValue;
use puniyu_session::EventSession;

/// 将 Extractor 和 Handler 组合为单一 Handler
pub struct CommandHandler<H: Handler , E: Extractor> {
	pub handler: H,
	pub extractor: E,
}


impl<H: Handler, E: Extractor> CommandHandler<H, E> {
	pub fn new(handler: H, extractor: E) -> Self {
		Self { handler, extractor }
	}
}

#[async_trait]
impl<H: Handler, E: Extractor> Handler for CommandHandler<H, E> {
	async fn handle(&self, session: EventSession, _params: ParamValue) -> puniyu_error::AnyError {
		let message = session.as_message()
			.ok_or("CommandHandler requires a message session")?;
		let params = self.extractor.extract(&message).await
			.unwrap_or(ParamValue::Empty);
		self.handler.handle(session, params).await
	}
}

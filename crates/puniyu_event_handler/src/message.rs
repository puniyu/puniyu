use super::{Handler, HandlerResult};
use async_trait::async_trait;
use puniyu_event_core::Event;

pub struct MessageHandler;

#[async_trait]
impl Handler for MessageHandler {
	async fn handle(&self, element: &Event) -> HandlerResult {
		HandlerResult::Ok
	}

	fn name(&self) -> &str {
		"message"
	}
}

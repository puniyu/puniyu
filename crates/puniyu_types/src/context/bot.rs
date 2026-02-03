use crate::adapter::AdapterApi;
use crate::bot::Bot;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BotContext {
	bot: Arc<Bot>,
}

impl BotContext {
	pub fn new(bot: Arc<Bot>) -> Self {
		Self { bot }
	}
	pub fn api(&self) -> &AdapterApi {
		&self.bot.api
	}
}

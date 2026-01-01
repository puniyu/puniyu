use super::store::BotStore;
use actix_ws::Session;
use std::sync::LazyLock;
static BOT_STORE: LazyLock<BotStore> = LazyLock::new(BotStore::default);
pub struct BotRegistry;

impl BotRegistry {
	pub fn insert(bot_app: impl Into<String>, session: Session) {
		BOT_STORE.insert(bot_app.into(), session);
	}
	pub fn remove(bot_app: impl Into<String>) {
		BOT_STORE.remove(bot_app.into());
	}
}

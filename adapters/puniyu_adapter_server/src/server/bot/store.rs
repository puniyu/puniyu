use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use actix_ws::Session;

#[derive(Default)]
pub(crate) struct BotStore(Arc<RwLock<HashMap<String, Session>>>);

impl BotStore {

    pub(crate) fn insert(&self, bot_app: String, session: Session) {
        self.0.write().unwrap().insert(bot_app, session);
    }

    pub(crate) fn get(&self, bot_app: &str) -> Option<Session> {
        self.0.read().unwrap().get(bot_app).cloned()
    }
    
    pub(crate) fn remove(&self, bot_app: String) {
        self.0.write().unwrap().remove(&bot_app);
    }
}
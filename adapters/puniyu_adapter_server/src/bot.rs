use actix_ws::Session;
use puniyu_adapter::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Bot {
	pub session: Arc<Mutex<Session>>,
	pub adapter: Arc<AdapterInfo>,
	pub account: Arc<AccountInfo>,
	pub event: Event,
}

#[allow(dead_code)]
pub struct Event {
	pub event_id: Arc<str>,
	pub self_id: Arc<str>,
	pub user_id: Arc<str>,
	pub contact: ContactType,
	pub sender: SenderType,
	pub time: u64,
	pub message_id: Arc<str>,
}

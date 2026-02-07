mod message;

use std::sync::Arc;
use puniyu_adapter::macros::api;
use puniyu_adapter::adapter::AdapterApi;


#[api]
fn api() -> AdapterApi {
	let message_api = Arc::new(message::ServerMessageApi);
	AdapterApi::new(Default::default(), Default::default(), Default::default(), message_api)
}


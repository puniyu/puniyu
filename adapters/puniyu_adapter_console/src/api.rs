mod message;

use puniyu_adapter::adapter::AdapterApi;
use puniyu_adapter::macros::api;
use puniyu_core::Config;
use std::sync::{Arc, LazyLock};

pub(crate) static AVATAR_URL: LazyLock<String> = LazyLock::new(|| {
	let config = Config::app();
	let server = config.server();
	format!("http://{}:{}/logo.png", server.host(), server.port())
});

#[api]
fn api() -> AdapterApi {
	let message_api = Arc::new(message::ConsoleMessageApi);
	AdapterApi::new(Default::default(), Default::default(), Default::default(), message_api)
}

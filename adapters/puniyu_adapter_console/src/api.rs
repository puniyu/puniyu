mod message;
use bytes::Bytes;
use puniyu_adapter::prelude::*;
use std::sync::{Arc, LazyLock};

pub(crate) static AVATAR: LazyLock<Bytes> = LazyLock::new(|| {
	let logo_path = resource_dir().join("logo.png");
	std::fs::read(logo_path).unwrap_or_default().into()
});

pub(crate) fn api() -> AdapterApi {
	let message_api = Arc::new(message::ConsoleMessageApi);
	AdapterApi::builder().message_api(message_api).build().unwrap()
}

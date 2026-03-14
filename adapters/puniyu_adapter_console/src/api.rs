mod message;

use puniyu_adapter_api::prelude::*;
use std::sync::{Arc, LazyLock};
use bytes::Bytes;

pub(crate) static AVATAR: LazyLock<Vec<u8>> = LazyLock::new(|| {
	let logo_path = resource_dir().join("logo.png");
	std::fs::read(logo_path).unwrap_or_default()
});

mod group {
	pub(crate) struct  ConsoleGroupApi;
	use puniyu_adapter_api::prelude::*;
	impl GroupApi for ConsoleGroupApi {
	}
}

mod friend {
	pub(crate) struct  ConsoleFriendApi;
	use puniyu_adapter_api::prelude::*;
	impl FriendApi for ConsoleFriendApi {
	}
}

mod account {
	pub(crate) struct ConsoleAccountApi;
	use puniyu_adapter_api::prelude::*;
	impl AccountApi for ConsoleAccountApi {
	}
}
pub(crate) fn api() -> AdapterApi {
	let group_api = Arc::new(group::ConsoleGroupApi);
	let friend_api = Arc::new(friend::ConsoleFriendApi);
	let account_api = Arc::new(account::ConsoleAccountApi);
	let message_api = Arc::new(message::ConsoleMessageApi);
	AdapterApi::new(group_api, friend_api, account_api, message_api)
}


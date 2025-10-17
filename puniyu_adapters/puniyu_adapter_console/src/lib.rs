mod api;

use async_trait::async_trait;
use puniyu_adapter::prelude::*;
use puniyu_core::APP_NAME;
use std::env;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

static EVENT_ID: AtomicU64 = AtomicU64::new(0);
static MESSAGE_ID: AtomicU64 = AtomicU64::new(0);

#[adapter]
pub struct Adapter;

#[async_trait]
impl AdapterBuilder for Adapter {
	fn info(&self) -> AdapterInfo {
		use std::time::SystemTime;
		use std::time::UNIX_EPOCH;
		let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
		adapter_info!(
			name: "Console",
			platform: AdapterPlatform::Other,
			standard: AdapterStandard::Other,
			protocol: AdapterProtocol::Console,
			communication: AdapterCommunication::Other,
			connect_time: start_time
		)
	}

	fn api(&self) -> &'static dyn AdapterApi {
		&api::ConsoleAdapterApi
	}

	fn abi_version(&self) -> &'static str {
		puniyu_adapter::ABI_VERSION
	}

	async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
		let bot_id = "console";
		let name = APP_NAME.get().unwrap();
		let account_info = account_info!(
			uin: bot_id,
			name: format!("{}/{}", name, bot_id),
			avatar: "".to_string()
		);
		register_bot!(self.info(), account_info);
		thread::spawn(move || {
			loop {
				let message = {
					let mut input = String::new();
					std::io::stdin().read_line(&mut input).unwrap();
					input.trim_end().to_string()
				};

				if matches!(message.as_str(), "quit" | "exit" | "q") {
					std::process::exit(0);
				}

				let contact = contact_friend!(name, name);
				let sender = friend_sender!(user_id: name, nick: name, sex: Sex::Unknown, age: 0);

				let event_id = EVENT_ID.fetch_add(1, Ordering::Relaxed).to_string();
				let message_id = MESSAGE_ID.fetch_add(1, Ordering::Relaxed).to_string();

				let adapter = Arc::new(api::ConsoleAdapterApi) as Arc<dyn AdapterApi>;

				create_friend_message!(
					adapter,
					event_id,
					contact,
					bot_id,
					bot_id,
					message_id,
					vec![element!(text, message)],
					sender
				);
			}
		});
		Ok(())
	}
}

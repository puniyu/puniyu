mod api;

use puniyu_core::adapter::prelude::*;
use std::env;
use std::thread;

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
					input.trim().to_string()
				};
				if message == "quit" {
					std::process::exit(0);
				}

				create_friend_message!("console_event", bot_id, "user", "msg_1", vec![]);
			}
		});
		Ok(())
	}
}

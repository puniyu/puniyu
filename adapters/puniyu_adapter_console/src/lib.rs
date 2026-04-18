mod common;
mod input;
mod runtime;
pub use runtime::ConsoleAdapterRuntime as Runtime;

use log::info;
use puniyu_adapter::app_name;
use puniyu_adapter::bot::get_bot;
use puniyu_adapter::macros::*;
use std::sync::Arc;

pub(crate) const VERSION: puniyu_adapter::Version = pkg_version!();
pub(crate) const NAME: &str = pkg_name!();

#[adapter(runtime = runtime::runtime)]
async fn main() -> puniyu_adapter::Result {
	let bot_id = "console";
	let name = app_name();
	let adapter_runtime: Arc<dyn puniyu_adapter::runtime::AdapterRuntime> =
		Arc::new(runtime::ConsoleAdapterRuntime::new());
	let bot = Arc::new(puniyu_adapter::bot::Bot::new(
		Arc::clone(&adapter_runtime),
		account_info!(
			uin: bot_id,
			name: format!("{}/{}", name, bot_id),
			avatar: puniyu_server::get_logo(),
		),
	));
	let bot_index = register_bot!(bot: bot)?;

	info!(
		"{} v{} 初始化完成",
		adapter_runtime.adapter_info().name,
		adapter_runtime.adapter_info().version
	);

	let bot = get_bot(bot_index).expect("bot just registered");
	let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();

	std::thread::spawn(move || {
		use std::io::BufRead;
		let stdin = std::io::stdin();
		for line in stdin.lock().lines() {
			match line {
				Ok(s) => {
					let _ = tx.send(s);
				}
				Err(_) => break,
			}
		}
	});

	tokio::spawn(async move {
		while let Some(message) = rx.recv().await {
			if matches!(message.as_str(), "quit" | "exit" | "q") {
				break;
			}

			let parsed = input::parse_console_input(&message);
			common::dispatch_event(bot.as_ref(), &parsed, name).await;
		}
	});

	Ok(())
}

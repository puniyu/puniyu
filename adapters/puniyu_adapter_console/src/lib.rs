mod bot;
mod common;
mod input;
mod runtime;
pub use runtime::ConsoleAdapterRuntime as Runtime;

use log::info;
use puniyu_adapter::app_name;
use puniyu_adapter::bot::get_bot;
use puniyu_adapter::macros::*;
use puniyu_runtime::AdapterProvider;
use std::sync::Arc;

pub(crate) const VERSION: puniyu_adapter::Version = pkg_version!();
pub(crate) const NAME: &str = pkg_name!();

#[adapter(runtime = runtime::runtime)]
async fn main() -> puniyu_adapter::Result {
	let bot_id = "console";
	let name = app_name();
	let adapter_runtime = Arc::new(runtime::ConsoleAdapterRuntime::new());
	let bot_runtime = Arc::new(runtime::ConsoleBotRuntime::new(
		Arc::clone(&adapter_runtime),
		account_info!(
			uin: bot_id,
			name: format!("{}/{}", name, bot_id),
			avatar: runtime::AVATAR.clone()
		),
	));
	let bot: Arc<dyn puniyu_adapter::bot::Bot> = Arc::new(bot::ConsoleBot::new(bot_runtime));
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
		loop {
			let message = tokio::select! {
				_ = tokio::signal::ctrl_c() => break,
				msg = rx.recv() => match msg {
					Some(s) if !matches!(s.as_str(), "quit" | "exit" | "q") => s,
					_ => break,
				},
			};

			let parsed = input::parse_console_input(&message);
			common::dispatch_event(bot.as_ref(), &parsed, name).await;
		}
	});

	Ok(())
}

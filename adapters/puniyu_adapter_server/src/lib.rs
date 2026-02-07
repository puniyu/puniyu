mod api;
mod bot;
mod error;
mod server;

use puniyu_adapter::prelude::*;
use puniyu_core::logger::info;

fn info() -> AdapterInfo {
	adapter_info!(
		platform: AdapterPlatform::Other,
		standard: AdapterStandard::Other,
		protocol: AdapterProtocol::Other,
		communication: AdapterCommunication::Other
	);
}
#[adapter(info = "info")]
async fn main() -> HandlerResult {
	use puniyu_core::Config;
	let config = Config::app();
	let server = config.server();
	info!("{} v{} 初始化完成", info().name, info().version);
	info!("服务端适配器连接地址: ws://{}:{}/api/bot/{{bot_app}}ws", server.host(), server.port());
	Ok(())
}

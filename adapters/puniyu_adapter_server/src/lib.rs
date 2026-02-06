mod api;
mod bot;
mod error;
mod server;

use actix_web::web::{self, ServiceConfig};
use async_trait::async_trait;
use puniyu_adapter::prelude::*;
use puniyu_core::Config;
use puniyu_core::logger::info;
use server::ws_handler;
use std::env;
use std::sync::Arc;

#[adapter]
struct Server;

#[async_trait]
impl Adapter for Server {
	fn name(&self) -> &'static str {
		env!("CARGO_PKG_NAME")
	}

	fn api(&self) -> AdapterApi {
		let group_api = Arc::new(api::ServerGroupApi);
		let friend_api = Arc::new(api::ServerFriendApi);
		let message_api = Arc::new(api::ServerMessageApi::default());
		let account_api = Arc::new(api::ServerAccountApi);
		AdapterApi::new(group_api, friend_api, account_api, message_api)
	}

	fn server(&self) -> Option<ServerFunction> {
		let server = |cfg: &mut ServiceConfig| {
			cfg.service(web::scope("/api/bot").configure(|cfg: &mut ServiceConfig| {
				cfg.route("/{bot_app}", web::get().to(ws_handler));
				cfg.route("/{bot_app}/ws", web::get().to(ws_handler));
			}));
		};
		Some(Arc::new(server))
	}

	async fn init(&self) -> Result<()> {
		let config = Config::app();
		let server = config.server();
		info!("{} v{} 初始化完成", self.name(), self.version());
		info!(
			"服务端适配器连接地址: ws://{}:{}/api/bot/{{bot_app}}ws",
			server.host(),
			server.port()
		);
		Ok(())
	}
}

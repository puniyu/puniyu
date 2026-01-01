mod api;
mod common;
mod server;

use async_trait::async_trait;
use puniyu_adapter::actix_web::web;
use puniyu_adapter::prelude::*;
use puniyu_adapter::{Result, ServerType, ServiceConfig};
use std::env;
use puniyu_core::logger::info;
use puniyu_core::Config;
use server::bot;

#[adapter]
struct Server;

#[async_trait]
impl AdapterBuilder for Server {
	fn name(&self) -> &'static str {
		env!("CARGO_PKG_NAME")
	}

	fn api(&self) -> &'static dyn AdapterApi {
		&api::ServerAdapterApi
	}

	fn server(&self) -> Option<ServerType> {
		let server = |cfg: &mut ServiceConfig| {
			cfg.service(web::scope("/api/bot").configure(|cfg: &mut ServiceConfig| {
				cfg.route("/{bot_app}", web::get().to(bot::ws_handler));
				cfg.route("/{bot_app}/ws", web::get().to(bot::ws_handler));
			}));
		};
		Some(std::sync::Arc::new(server))
	}

	async fn init(&self) -> Result<()> {
		let config = Config::app();
		let server = config.server();
		info!("{} v{} 初始化完成", self.name(), self.version());
		info!("服务端适配器连接地址: ws://{}:{}", server.host(), server.port());
		Ok(())
	}
}

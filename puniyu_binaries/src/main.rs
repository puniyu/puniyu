#![allow(clippy::unwrap_used)]

mod load;
mod log;

use std::io;

use bytes::Bytes;
use salvo::Router;
use semver::Version;

pub(crate) const NAME: &str = "puniyu";
pub(crate) const VERSION: Version = puniyu_version::VERSION;
pub(crate) const ASSETS: &[u8] =
	include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/logo.png"));

#[tokio::main]
async fn main() -> io::Result<()> {
	puniyu::App::builder()
		.name(NAME)
		.handler(puniyu_handler_log::LogHandler)
		.handler(puniyu_handler_access::AccessHandler)
		.handler(puniyu_handler_command::CommandHandler)
		.router(Router::with_path("logo").get(puniyu_server_router::logo::logo))
		.hoop(puniyu_server_middleware::AccessLog)
		.hoop(puniyu_server_middleware::Logo::new(Bytes::from_static(ASSETS)))
		.on_start(load::load)
		.build()
		.run()
		.await
}

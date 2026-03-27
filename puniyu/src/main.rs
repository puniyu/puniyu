mod logger;

use crate::logger::log_init;
use bytes::Bytes;
use puniyu_core::App;

#[tokio::main]
async fn main() -> std::io::Result<()> {
	log_init();
	App::builder()
		.with_app_logo(Bytes::from_static(include_bytes!("../assets/logo.png")))
		.build()
		.run()
		.await
}

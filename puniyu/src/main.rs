mod logger;

use puniyu_core::App;
use bytes::Bytes;
use crate::logger::log_init;

#[tokio::main]
async fn main() -> std::io::Result<()> {
	log_init();
	App::builder()
		.with_app_logo(Bytes::from_static(include_bytes!("../assets/logo.png")))
		.build()
		.run()
		.await
}

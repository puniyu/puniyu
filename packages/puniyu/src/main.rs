use bytes::Bytes;
use puniyu_core::App;

#[tokio::main]
async fn main() -> std::io::Result<()> {
	App::builder()
		.with_app_logo(Bytes::from_static(include_bytes!("../assets/logo.png")))
		.build()
		.run()
		.await
}

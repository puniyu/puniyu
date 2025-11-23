use puniyu_core::AppBuilder;

#[tokio::main]
async fn main() {
	const LOGO: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/logo.png"));
	let app = AppBuilder::new()
		.with_logo(LOGO.to_vec())
		.with_adapter(&puniyu_adapter_console::Adapter)
		.build();
	app.run().await;
}

use bytes::Bytes;
use puniyu_core::App;

#[tokio::main]
async fn main() -> std::io::Result<()> {
	App::builder()
		.with_app_logo(Bytes::from_static(include_bytes!("../assets/logo.png")))
		.with_handler(puniyu_handler_command::Handler)
		.with_adapter(puniyu_adapter_console::Adapter)
		.with_plugin(puniyu_plugin_basic::Plugin)
		.build()
		.run()
		.await
}

use puniyu_core::App;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
	App::builder()
		.with_adapter(&puniyu_adapter_console::Adapter)
		.with_plugin(&puniyu_plugin_basic::Plugin)
		.with_adapter(&puniyu_adapter_server::Adapter)
		.build()
		.run()
		.await?;
	Ok(())
}

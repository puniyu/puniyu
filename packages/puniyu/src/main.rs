use puniyu_core::AppBuilder;

#[tokio::main]
async fn main() {
	let app = AppBuilder::default()
		.with_adapter(&puniyu_adapter_console::Adapter)
		.with_plugin(&puniyu_plugin_basic::Plugin)
		.with_adapter(&puniyu_adapter_server::Adapter)
		.build();
	app.run().await;
}

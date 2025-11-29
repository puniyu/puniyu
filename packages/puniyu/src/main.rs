use puniyu_core::AppBuilder;

#[tokio::main]
async fn main() {
	let app = AppBuilder::new()
		.with_adapter(&puniyu_adapter_console::Adapter)
		.with_plugin(&puniyu_plugin_echo::Plugin)
		.build();
	app.run().await;
}

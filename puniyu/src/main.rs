use puniyu_core::App;
#[tokio::main]
async fn main() {
	let mut app = App::default();
	app.add_plugin(&puniyu_plugin_test::PluginInfo);
	app.run().await;
}

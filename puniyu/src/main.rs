use puniyu_core::App;
#[tokio::main]
async fn main() {
	let mut app = App::default();
	app.add_adapter(&puniyu_adapter_console::Adapter);
	app.run().await;
}

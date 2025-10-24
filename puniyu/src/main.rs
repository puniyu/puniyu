use puniyu_core::App;
#[tokio::main]
async fn main() {
	let mut app = App::default();
	app.run().await;
}

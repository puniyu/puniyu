use puniyu_core::App;
#[tokio::main]
async fn main() {
	let app = App::default();
	app.run().await;
}

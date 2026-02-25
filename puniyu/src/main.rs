use puniyu_core::App;

#[tokio::main]
async fn main() -> std::io::Result<()> {
	App::builder().build().run().await
}

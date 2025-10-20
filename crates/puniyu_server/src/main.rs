#[tokio::main]
async fn main() {
	puniyu_server::run_server(None, None).await;
}

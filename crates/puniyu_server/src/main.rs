#[tokio::main]
async fn main() -> std::io::Result<()> {
	puniyu_server::run_server(None, None).await?;
	Ok(())
}

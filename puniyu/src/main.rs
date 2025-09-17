use puniyu_core::app::Bot;
#[tokio::main(worker_threads = 4)]
async fn main() {
	let bot = Bot::new().daemon();
	bot.run().await;
}

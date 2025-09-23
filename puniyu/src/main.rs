use puniyu_core::app::Bot;
#[tokio::main]
async fn main() {
	let bot = Bot::default();
	bot.run().await;
}

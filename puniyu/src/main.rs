use puniyu_core::bot::Bot;

#[tokio::main(worker_threads = 4)]
async fn main() {
    let bot = Bot::new();
    bot.run().await;
}

use puniyu_core::logger::init_logger;
use puniyu_plugin::task::init_task;
#[tokio::main]
async fn main() {
    init_logger("info".to_string(), None);
    init_task().await;
}

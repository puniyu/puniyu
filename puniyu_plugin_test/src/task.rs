use puniyu_registry::task;

#[task(cron = "0 * * * * *")]
pub async fn test() {
    println!("test");
}

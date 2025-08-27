use puniyu_registry::task;
#[task(cron = "@yearly")]
async fn min() {
    log::info!("test");
}

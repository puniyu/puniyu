use log::info;
use puniyu_registry::task;

#[task(cron = "* * * * * *")]
pub async fn test() {
    let msg = "你个猪咪";
    info!("{}", msg)
}

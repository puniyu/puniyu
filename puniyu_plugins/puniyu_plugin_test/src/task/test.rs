use puniyu_registry::logger::owo_colors::OwoColorize;
use puniyu_registry::task;

#[task(cron = "* * * * * *")]
pub async fn test() {
	let msg = "你个猪咪".red().to_string();
	info!("{}", msg)
}

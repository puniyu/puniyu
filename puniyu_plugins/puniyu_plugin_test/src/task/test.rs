use puniyu_core::APP_NAME;
use puniyu_core::plugin::prelude::*;

#[task(cron = "* * * * * *")]
pub async fn test() {
	let name = APP_NAME.get().unwrap();
	println!("{}", name);
}

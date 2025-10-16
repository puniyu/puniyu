use puniyu_core::plugin::prelude::*;

#[task(cron = "* * * * * *")]
async fn test() {
	println!("hello");
}

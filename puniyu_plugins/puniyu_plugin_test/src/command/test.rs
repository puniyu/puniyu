use puniyu_plugin::prelude::*;

#[command(
name = "echo",
args = ["name"],
rank = 50,
)]
async fn test(bot: &Bot, event: &EventContext) -> HandlerResult {
	bot.reply(Message::from("hello"));
	let n = event.arg("name").unwrap();
	println!("参数: {}", n);
	HandlerResult::Ok
}

use puniyu_plugin::prelude::*;

#[command(
name = "echo",
args = ["name"],
rank = 50,
)]
async fn test(_bot: &BotContext, event: &MessageContext) -> HandlerResult {
	let n = event.arg("name").unwrap();
	println!("参数: {}", n);
	HandlerResult::Ok
}

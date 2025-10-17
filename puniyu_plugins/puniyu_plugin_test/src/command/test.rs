use puniyu_plugin::prelude::*;

#[command(name = "echo", rank = "5")]
async fn test(bot: &Bot, event: &EventContext) -> HandlerResult {
	bot.reply(Message::from("hello"));
	HandlerResult::Ok
}

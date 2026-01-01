mod registry;
mod store;

use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_ws::Message;
use futures_util::StreamExt;
use puniyu_core::logger::info;
use puniyu_protocol::event::EventSend;
use registry::BotRegistry;

//todo: 完善注册的连接
pub async fn ws_handler(
	req: HttpRequest,
	body: web::Payload,
	path: web::Path<String>,
) -> Result<HttpResponse, Error> {
	let bot_name = path.into_inner();
	info!("Bot {} 正在连接...", bot_name);

	let (response, session, mut msg_stream) = actix_ws::handle(&req, body)?;

	BotRegistry::insert(bot_name.clone(), session);
	info!("Bot {} 已连接", bot_name);
	actix_web::rt::spawn(async move {
		while let Some(Ok(msg)) = msg_stream.next().await {
			match msg {
				Message::Text(text) => {
					info!("[Bot {}] 收到: {}", bot_name, text);
				}
				Message::Binary(binary) => {
				}
				Message::Close(reason) => {
					info!("[Bot {}] 断开连接: {:?}", bot_name, reason);
					BotRegistry::remove(&bot_name);
					break;
				}
				_ => {}
			}
		}
		BotRegistry::remove(&bot_name)
	});

	Ok(response)
}

use actix_web::{Error, HttpRequest, HttpResponse, web};
use futures_util::StreamExt;
use prost::Message;
use puniyu_adapter::logger::{error, info};
use puniyu_protocol::event::EventReceive;
use std::sync::Arc;

mod event;

pub async fn ws_handler(
	req: HttpRequest,
	body: web::Payload,
	path: web::Path<String>,
) -> Result<HttpResponse, Error> {
	let bot_name = path.into_inner();
	info!("Bot {} 正在连接...", bot_name);

	let (response, session, mut msg_stream) = actix_ws::handle(&req, body)?;

	let session = Arc::new(tokio::sync::Mutex::new(session));
	info!("Bot {} 已连接", bot_name);
	actix_web::rt::spawn(async move {
		while let Some(Ok(msg)) = msg_stream.next().await {
			match msg {
				actix_ws::Message::Binary(binary) => {
					let event = EventReceive::decode(binary);
					if let Ok(event) = event {
						return event::handle_event(event, &session)
					}
					error!("解析Bot {} 事件失败", bot_name);
				}
				actix_ws::Message::Close(reason) => {
					info!("[Bot {}] 断开连接: {:?}", bot_name, reason);
					break;
				}
				_ => {}
			}
		}
	});

	Ok(response)
}

use actix_web::{Error, HttpRequest, HttpResponse, web};
use futures_util::StreamExt;
use prost::Message;
use puniyu_adapter::logger::{error, info};
use puniyu_probuff::event::EventReceive;
use std::sync::Arc;
use actix_web::web::ServiceConfig;

mod connection;
mod event;
mod store;
pub use connection::ConnectionManager;
use puniyu_adapter::macros::server;
use puniyu_adapter::server::ServerFunction;

async fn ws_handler(
	req: HttpRequest,
	body: web::Payload,
	path: web::Path<String>,
) -> Result<HttpResponse, Error> {
	let bot_name = path.into_inner();
	info!("Bot {} 正在连接...", &bot_name);

	let (response, session, mut msg_stream) = actix_ws::handle(&req, body)?;

	let session = Arc::new(tokio::sync::Mutex::new(session));

	ConnectionManager::add(bot_name.clone(), Arc::clone(&session));
	info!("Bot {} 已连接", bot_name);

	actix_web::rt::spawn(async move {
		while let Some(Ok(msg)) = msg_stream.next().await {
			match msg {
				actix_ws::Message::Binary(binary) => {
					if let (Some(session), Ok(event)) =
						(ConnectionManager::get(&bot_name), EventReceive::decode(binary))
					{
						return event::handle_event(event, &session);
					}
					error!("解析Bot {} 事件失败", bot_name);
				}
				actix_ws::Message::Close(reason) => {
					match reason {
						Some(r) => info!("[Bot {}] 断开连接: {:?}", bot_name, r),
						None => info!("[Bot {}] 断开连接", bot_name),
					}
					ConnectionManager::remove(&bot_name);
					break;
				}
				_ => {}
			}
		}
	});

	Ok(response)
}

#[server]
fn server(cfg: &mut ServiceConfig) -> ServerFunction {
	let server= cfg.service(web::scope("/api/bot").configure(|cfg| {
		cfg.route("/{bot_app}", web::get().to(ws_handler));
		cfg.route("/{bot_app}/ws", web::get().to(ws_handler));
	}));
	Arc::new(server)
}
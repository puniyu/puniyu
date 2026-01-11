use super::connection::ConnectionManager;
use prost::Message;
use puniyu_adapter::prelude::*;
use puniyu_protocol::event::EventReceive;

#[hook(type = "event.message")]
async fn event_server(ev: Option<&Event>) -> HandlerResult {
	let Some(event) = ev else {
		return Ok(());
	};
	let connections = ConnectionManager::get_all();
	let pb: EventReceive = event.clone().into();
	let bin = pb.encode_to_vec();
	for (_bot_name, session) in connections {
		let mut session = session.lock().await;
		session.binary(bin.clone()).await?;
	}
	Ok(())
}

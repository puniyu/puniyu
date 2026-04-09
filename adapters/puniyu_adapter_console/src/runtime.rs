use bytes::Bytes;
use puniyu_adapter::prelude::*;
use std::sync::LazyLock;
use crate::common::make_random_id;
use async_trait::async_trait;
use log::debug;

pub(crate) static AVATAR: LazyLock<Bytes> = LazyLock::new(|| {
	let logo_path = resource_dir().join("logo.png");
	std::fs::read(logo_path).unwrap_or_default().into()
});

use std::time::{SystemTime, UNIX_EPOCH};

pub struct ConsoleRuntime;


#[async_trait]
impl Runtime for ConsoleRuntime {
	async fn send_message(
		&self,
		contact: &ContactType<'_>,
		message: &Message,
	) -> puniyu_adapter::Result<SendMsgType> {
		let (msg_type, source) = match contact {
			ContactType::Friend(friend) => ("私聊消息", &friend.scene()),
			ContactType::Group(group) => ("群聊消息", &group.scene()),
			ContactType::GroupTemp(group) => ("群临时消息", &group.scene()),
		};
		let message_id = make_random_id();
		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map_err(Box::<dyn std::error::Error + Send + Sync>::from)?
			.as_secs();

		debug!("[发送{}:{}]\n{:#?}", msg_type, source, message);

		Ok(SendMsgType { message_id, time: timestamp })
	}
}

pub(crate) fn runtime() -> AdapterRuntime {
	AdapterRuntime::from_runtime(ConsoleRuntime)
}

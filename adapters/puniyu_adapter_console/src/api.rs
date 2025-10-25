use crate::common::make_message_id;
use async_trait::async_trait;
use puniyu_adapter::Result;
use puniyu_adapter::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

macro_rules! info {
			($($arg:tt)*) => {
				{
					use puniyu_adapter::logger::owo_colors::OwoColorize;
					let prefix = "adapter".fg_rgb::<176, 196, 222>();
					let func_name = env!("CARGO_PKG_NAME").fg_rgb::<255, 192, 203>();
					puniyu_adapter::logger::info!("[{}:{}] {}", prefix, func_name, format!($($arg)*))
				}
			};
		}

pub struct ConsoleAdapterApi;

#[async_trait]
impl AdapterApi for ConsoleAdapterApi {
	async fn get_avatar_url(&self, _target_id: &str, _size: Option<AvatarSize>) -> Result<String> {
		todo!()
	}

	async fn get_group_avatar_url(
		&self,
		_group_id: &str,
		_size: Option<AvatarSize>,
	) -> Result<String> {
		todo!()
	}

	async fn send_msg(&self, contact: Contact, element: Message) -> Result<SendMsgType> {
		let source = match contact {
			Contact::Friend(friend) => friend.scene,
			Contact::Group(group) => group.scene,
		};
		let message_id = make_message_id();
		let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

		info!("[发送消息:{}] {:?}", source.to_string(), element);

		Ok(SendMsgType { message_id, time: timestamp })
	}
}

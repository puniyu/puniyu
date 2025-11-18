use crate::common::make_message_id;
use async_trait::async_trait;
use puniyu_adapter::Result;
use puniyu_adapter::prelude::*;
use puniyu_core::Config;
use std::sync::LazyLock;
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

static AVATAR_URL: LazyLock<String> = LazyLock::new(|| {
	let server = Config::app().server();
	format!("http://{}:{}/logo.png", server.host(), server.port())
});
pub struct ConsoleAdapterApi;

#[async_trait]
impl AdapterApi for ConsoleAdapterApi {
	async fn get_avatar(&self, _target_id: &str, _size: Option<AvatarSize>) -> Result<Avatar> {
		Ok(AVATAR_URL.clone().into())
	}

	async fn get_group_avatar(&self, _group_id: &str, _size: Option<AvatarSize>) -> Result<Avatar> {
		Ok(AVATAR_URL.clone().into())
	}

	async fn send_msg(&self, contact: ContactType, element: Message) -> Result<SendMsgType> {
		let source = match contact {
			ContactType::Friend(friend) => friend.scene,
			ContactType::Group(group) => group.scene,
		};
		let message_id = make_message_id();
		let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

		info!("[发送消息:{}] {:?}", source.to_string(), element);

		Ok(SendMsgType { message_id, time: timestamp })
	}
}

use crate::adapter::AdapterInfo;
use crate::contact::Contact;
use crate::message::Message;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, PartialEq, EnumString, Display, IntoStaticStr, Deserialize, Serialize)]
pub enum AvatarSize {
	#[strum(serialize = "small")]
	Small,
	#[strum(serialize = "medium")]
	Medium,
	#[strum(serialize = "large")]
	Large,
}

/// 适配器基类
/// 开发者需要自行实现开发适配器，部分函数需要开发者自行实现
/// TODO: 未完成消息发送以及获取消息
///
///
#[async_trait]
pub trait AdapterBase: Send + Sync + 'static {
	/// 适配器信息
	fn info(&self) -> AdapterInfo;

	/// 获取适配器API
	fn api(&self) -> &'static dyn AdapterApi;

	/// 初始化
	async fn init(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait AdapterApi: Send + Sync + 'static {
	fn send_msg(&self, contact: Contact, element: Message);
	/// 获取头像URL
	fn get_avatar_url(&self, user_id: &str, size: AvatarSize) -> String;

	/// 获取群头像URL
	fn get_group_avatar_url(&self, group_id: &str, size: AvatarSize) -> String;
}

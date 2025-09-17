use crate::adapter::{AccountInfo, AdapterInfo};
use std::pin::Pin;

pub enum AvatarSize {
	Small,
	Medium,
	Large,
}

/// 适配器基类
/// 开发者需要自行实现开发适配器，部分函数需要开发者自行实现
/// TODO: 未完成消息发送以及获取消息
///
pub trait AdapterBase: Send + Sync + 'static {
	/// 获取适配器信息
	fn adapter(&self) -> AdapterInfo;
	/// 获取账户信息
	fn account(&self) -> AccountInfo;

	/// 获取适配器API
	fn api(&self) -> &'static dyn AdapterApi;

	/// 获取Bot账号的selfId
	///
	/// # 返回值
	///
	/// * `String` - Bot账号的selfId
	fn self_id(&self) -> &'static str {
		self.account().self_id
	}

	/// 获取Bot账号的昵称
	///
	/// # 返回值
	///
	/// * `String` - Bot账号的昵称
	fn self_name(&self) -> &'static str {
		self.account().name
	}

	fn init(&self) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
}

pub trait AdapterApi: Send + Sync + 'static {
	/// 获取群头像URL
	fn get_avatar_url(&self, user_id: &str, size: AvatarSize) -> String;

	fn get_group_avatar_url(&self, group_id: &str, size: AvatarSize) -> String;
}

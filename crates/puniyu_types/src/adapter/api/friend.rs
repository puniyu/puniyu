use std::any::Any;
use super::{AvatarSize, Error, Result, SetFriendApplyType, UserInfo, Avatar};
use async_trait::async_trait;

#[async_trait]
pub trait FriendApi: Send + Sync + Any {
	/// 获取头像
	///
	/// ## 参数
	/// `target_id` - 目标ID
	/// `size` - 头像尺寸
	///
	async fn get_avatar(&self, _target_id: &str, _size: Option<AvatarSize>) -> Result<Avatar> {
		Err(Error::NotImpl)
	}

	/// 获取好友列表
	async fn get_list(&self) -> Result<Vec<UserInfo>> {
		Err(Error::NotImpl)
	}

	/// 设置好友申请
	///
	/// ## 参数
	/// `action` - 设置或拒绝好友申请
	///
	async fn set_friend_apply(&self, _action: SetFriendApplyType) -> Result<()> {
		Err(Error::NotImpl)
	}
}

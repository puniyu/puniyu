use super::inner::Error;
use async_trait::async_trait;
use puniyu_adapter_types::{Avatar, AvatarSize, SetFriendApplyType, UserInfo};
use puniyu_error::Result;

#[async_trait]
pub trait FriendApi: Send + Sync {
	/// 获取头像
	///
	/// ## 参数
	/// `target_id` - 目标ID
	/// `size` - 头像尺寸
	///
	async fn get_user_avatar(&self, target_id: &str, size: Option<&AvatarSize>) -> Result<Avatar> {
		Err(Error::NotImpl.into())
	}

	/// 获取好友列表
	async fn get_friend_list(&self) -> Result<Vec<UserInfo>> {
		Err(Error::NotImpl.into())
	}

	/// 设置好友申请
	///
	/// ## 参数
	/// `action` - 设置或拒绝好友申请
	///
	async fn set_friend_apply(&self, action: &SetFriendApplyType) -> Result<()> {
		Err(Error::NotImpl.into())
	}
}

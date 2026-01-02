use super::{
	Avatar, AvatarSize, Error, GroupInfo, GroupMuteInfo, MuteType, Result, SetAdminType,
	SetGroupApplyType, UserInfo,
};
use async_trait::async_trait;
use std::time::Duration;

#[async_trait]
pub trait GroupApi: Send + Sync {
	/// 获取头像
	///
	/// ## 参数
	/// `group_id` - 目标群Id
	/// `size` - 头像尺寸
	///
	async fn get_group_avatar(&self, _group_id: &str, _size: Option<AvatarSize>) -> Result<Avatar> {
		Err(Error::NotImpl)
	}

	/// 获取群信息
	///
	/// ## 参数
	/// `group_id` - 群ID
	///
	async fn get_group_info(&self, _group_id: &str) -> Result<GroupInfo> {
		Err(Error::NotImpl)
	}

	/// 获取群列表
	async fn get_group_list(&self) -> Result<Vec<GroupInfo>> {
		Err(Error::NotImpl)
	}

	/// 获取群成员列表
	///
	/// ## 参数
	/// `group_id` - 群ID
	///
	async fn get_group_member_list(&self, _group_id: &str) -> Result<Vec<UserInfo>> {
		Err(Error::NotImpl)
	}

	/// 获取群禁言列表
	///
	/// ## 参数
	/// `group_id` - 群ID
	///
	async fn get_group_mute_list(&self, _group_id: &str) -> Result<Vec<GroupMuteInfo>> {
		Err(Error::NotImpl)
	}

	/// 设置群名称
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `name` - 群名称
	///
	async fn set_group_name(&self, _group_id: &str, _name: &str) -> Result<()> {
		Err(Error::NotImpl)
	}

	/// 群踢人
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `target_id` - 目标ID
	/// `reject_add_request` - 是否拒绝加群请求
	/// `reason` - 踢人原因
	///
	async fn set_kick_group_member(
		&self,
		_group_id: &str,
		_target_id: &str,
		_reject_add_request: Option<bool>,
		_reason: Option<&str>,
	) -> Result<()> {
		Err(Error::NotImpl)
	}

	/// 群禁言
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `target_id` - 目标ID
	/// `duration` - 禁言时长
	///
	async fn set_group_mute(&self, _group_id: &str, _target_id: &str, _duration: Duration) -> Result<()> {
		Err(Error::NotImpl)
	}
	/// 群全体禁言
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `action` - 设置或取消全体禁言
	async fn set_group_all_mute(&self, _group_id: &str, _action: MuteType) -> Result<()> {
		Err(Error::NotImpl)
	}

	/// 设置群管理员
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `target_id` - 目标ID
	/// `action` - 设置或取消管理员
	///
	async fn set_group_admin(
		&self,
		_group_id: &str,
		_target_id: &str,
		_action: SetAdminType,
	) -> Result<()> {
		Err(Error::NotImpl)
	}

	/// 退出群组
	///     如果Bot为群主，则解散群组
	/// ## 参数
	/// `group_id` - 群ID
	///
	async fn set_group_quit(&self, _group_id: &str) -> Result<()> {
		Err(Error::NotImpl)
	}

	/// 设置加群申请
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `action` - 设置或拒绝加群申请
	///
	async fn set_group_invited_join(&self, _group_id: &str, _action: SetGroupApplyType) -> Result<()> {
		Err(Error::NotImpl)
	}
}

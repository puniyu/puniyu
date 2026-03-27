use super::inner::Error;
use async_trait::async_trait;
use puniyu_adapter_types::{
	Avatar, AvatarSize, GroupInfo, GroupMuteInfo, MuteType, SetAdminType, SetGroupApplyType,
	UserInfo,
};
use puniyu_error::Result;
use std::time::Duration;

#[async_trait]
pub trait GroupApi: Send + Sync {
	/// 获取群头像。
	async fn get_group_avatar(&self, group_id: &str, size: Option<&AvatarSize>) -> Result<Avatar> {
		Err(Error::NotImpl.into())
	}

	/// 获取群信息。
	async fn get_group_info(&self, group_id: &str) -> Result<GroupInfo> {
		Err(Error::NotImpl.into())
	}

	/// 获取群列表。
	async fn get_group_list(&self) -> Result<Vec<GroupInfo>> {
		Err(Error::NotImpl.into())
	}

	/// 获取群成员列表。
	async fn get_group_member_list(&self, group_id: &str) -> Result<Vec<UserInfo>> {
		Err(Error::NotImpl.into())
	}

	/// 获取群禁言列表。
	async fn get_group_mute_list(&self, group_id: &str) -> Result<Vec<GroupMuteInfo>> {
		Err(Error::NotImpl.into())
	}

	/// 设置群名称。
	async fn set_group_name(&self, group_id: &str, name: &str) -> Result<()> {
		Err(Error::NotImpl.into())
	}

	/// 踢出群成员。
	async fn set_kick_group_member(
		&self,
		group_id: &str,
		target_id: &str,
		reject_add_request: Option<bool>,
		reason: Option<&str>,
	) -> Result<()> {
		Err(Error::NotImpl.into())
	}

	/// 设置群成员禁言。
	async fn set_group_mute(
		&self,
		group_id: &str,
		target_id: &str,
		duration: &Duration,
	) -> Result<()> {
		Err(Error::NotImpl.into())
	}

	/// 设置全体禁言。
	async fn set_group_all_mute(&self, group_id: &str, action: &MuteType) -> Result<()> {
		Err(Error::NotImpl.into())
	}

	/// 设置群管理员。
	async fn set_group_admin(
		&self,
		group_id: &str,
		target_id: &str,
		action: &SetAdminType,
	) -> Result<()> {
		Err(Error::NotImpl.into())
	}

	/// 退出群组；若 Bot 为群主则解散群组。
	async fn set_group_quit(&self, group_id: &str) -> Result<()> {
		Err(Error::NotImpl.into())
	}

	/// 处理入群申请。
	async fn set_group_invited_join(
		&self,
		group_id: &str,
		action: &SetGroupApplyType,
	) -> Result<()> {
		Err(Error::NotImpl.into())
	}
}

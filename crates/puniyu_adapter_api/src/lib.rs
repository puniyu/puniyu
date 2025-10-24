pub mod types;

use crate::types::{
	AvatarSize, CreateGroupFolderInfo, DownloadFileInfo, GroupHighlightsType, GroupInfo,
	GroupMuteInfo, HighlightsAction, MessageType, MuteType, QQCredentialInfo,
	QQGroupFileSystemInfo, QQRkeyInfo, SendMsgType, SetAdminType, SetFriendApplyType,
	SetGroupApplyType, UserInfo,
};
use async_trait::async_trait;
use puniyu_contact::Contact;
use puniyu_element::Message;
use std::path::PathBuf;
use std::time::Duration;

pub enum GetHistoryMsgType {
	MessageId(String),
	MessageSeq(u64),
}

impl From<String> for GetHistoryMsgType {
	fn from(message_id: String) -> Self {
		Self::MessageId(message_id)
	}
}
impl From<&str> for GetHistoryMsgType {
	fn from(message_id: &str) -> Self {
		Self::MessageId(String::from(message_id))
	}
}

impl From<u64> for GetHistoryMsgType {
	fn from(message_seq: u64) -> Self {
		Self::MessageSeq(message_seq)
	}
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[async_trait]
pub trait AdapterApi: Send + Sync + 'static {
	/// 获取头像URL
	///
	/// ## 参数
	/// `target_id` - 目标ID
	/// `size` - 头像尺寸
	///
	async fn get_avatar_url(&self, _target_id: &str, _size: Option<AvatarSize>) -> Result<String> {
		Err("此接口未实现".into())
	}

	/// 获取群头像URL
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `size` - 头像尺寸
	///
	async fn get_group_avatar_url(
		&self,
		_group_id: &str,
		_size: Option<AvatarSize>,
	) -> Result<String> {
		Err("此接口未实现".into())
	}

	/// 发送消息
	///
	/// ## 参数
	/// `contact` - 联系人
	/// `element` - 消息元素
	///
	async fn send_msg(&self, _contact: Contact, _element: Message) -> Result<SendMsgType> {
		Err("此接口未实现".into())
	}

	/// 发送长消息
	///
	/// ## 参数
	/// `res_id` - 资源ID
	///
	async fn send_long_msg(&self, _res_id: &str) -> Result<SendMsgType> {
		Err("此接口未实现".into())
	}

	/// 撤回消息
	///
	/// ## 参数
	/// `message_id` - 消息ID
	///
	async fn recall_msg(&self, _message_id: &str) -> Result<()> {
		Err("此接口未实现".into())
	}

	/// 获取消息
	///
	/// ## 参数
	/// `message_id` - 消息ID
	///
	async fn get_msg(&self, _message_id: &str) -> Result<MessageType> {
		Err("此接口未实现".into())
	}

	/// 获取历史消息
	///     指定消息位置开始向前获取历史消息，按时间倒序排列
	///
	/// ## 参数
	/// `contact` - 联系人
	/// `message` - 消息ID或消息序列号
	/// `count` - 获取消息数量，最大值为20
	///
	async fn get_history_msg(
		&self,
		_contact: Contact,
		_message: GetHistoryMsgType,
		_count: u8,
	) -> Result<Vec<MessageType>> {
		Err("此接口未实现".into())
	}

	/// 获取群精华消息
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `page` - 页码，从1开始
	/// `page_size` - 每页数量，最大值为20
	///
	async fn get_group_highlights(
		&self,
		_group_id: &str,
		_page: u8,
		_page_size: u8,
	) -> Result<Vec<GroupHighlightsType>> {
		Err("此接口未实现".into())
	}

	/// 设置群精华消息
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `message_id` - 消息ID
	/// `action` - 添加或删除精华消息
	///
	async fn set_group_highlights(
		&self,
		_group_id: &str,
		_message_id: &str,
		_action: HighlightsAction,
	) -> Result<()> {
		Err("此接口未实现".into())
	}

	/// 发送赞
	///
	/// ## 参数
	/// `target_id` - 目标ID
	/// `count` - 赞的数量，默认为10
	///
	async fn send_like(&self, _target_id: &str, _count: Option<u8>) -> Result<()> {
		Err("此接口未实现".into())
	}
	/// 群踢人
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `target_id` - 目标ID
	/// `reject_add_request` - 是否拒绝加群请求
	/// `reason` - 踢人原因
	///
	async fn group_kick_member(
		&self,
		_group_id: &str,
		_target_id: &str,
		_reject_add_request: Option<bool>,
		_reason: Option<&str>,
	) -> Result<()> {
		Err("此接口未实现".into())
	}

	/// 群禁言
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `target_id` - 目标ID
	/// `duration` - 禁言时长
	///
	async fn set_group_mute(
		&self,
		_group_id: &str,
		_target_id: &str,
		_duration: Duration,
	) -> Result<()> {
		Err("此接口未实现".into())
	}

	/// 群全体禁言
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `action` - 设置或取消全体禁言
	async fn set_group_all_mute(&self, _group_id: &str, _action: MuteType) -> Result<()> {
		Err("此接口未实现".into())
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
		Err("此接口未实现".into())
	}

	/// 设置群成员名片
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `target_id` - 目标ID
	/// `card` - 名片内容
	///
	async fn set_group_member_card(
		&self,
		_group_id: &str,
		_target_id: &str,
		_card: &str,
	) -> Result<()> {
		Err("此接口未实现".into())
	}

	/// 设置群名称
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `name` - 群名称
	///
	async fn set_group_name(&self, _group_id: &str, _name: &str) -> Result<()> {
		Err("此接口未实现".into())
	}

	/// 退出群组
	///     如果Bot为群主，则解散群组
	/// ## 参数
	/// `group_id` - 群ID
	///
	async fn set_group_quit(&self, _group_id: &str) -> Result<()> {
		Err("此接口未实现".into())
	}

	/// 设置群成员头衔
	///     仅Bot为群主可用
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `target_id` - 目标ID
	/// `title` - 头衔内容
	///
	async fn set_group_member_title(
		&self,
		_group_id: &str,
		_target_id: &str,
		_title: &str,
	) -> Result<()> {
		Err("此接口未实现".into())
	}

	/// 获取陌生人信息
	///
	/// ## 参数
	/// `target_id` - 陌生人ID
	///
	async fn get_stranger_info(&self, _target_id: &str) -> Result<UserInfo> {
		Err("此接口未实现".into())
	}

	/// 获取好友列表
	async fn get_friend_list(&self) -> Result<Vec<UserInfo>> {
		Err("此接口未实现".into())
	}

	/// 获取群信息
	///
	/// ## 参数
	/// `group_id` - 群ID
	///
	async fn get_group_info(&self, _group_id: &str) -> Result<GroupInfo> {
		Err("此接口未实现".into())
	}

	/// 获取群列表
	async fn get_group_list(&self) -> Result<Vec<GroupInfo>> {
		Err("此接口未实现".into())
	}

	/// 获取群成员列表
	///
	/// ## 参数
	/// `group_id` - 群ID
	///
	async fn get_group_member_list(&self, _group_id: &str) -> Result<Vec<UserInfo>> {
		Err("此接口未实现".into())
	}

	async fn get_group_honor(&self, _group_id: &str) -> Result<Vec<UserInfo>> {
		Err("此接口未实现".into())
	}

	/// 设置好友申请
	///
	/// ## 参数
	/// `action` - 设置或拒绝好友申请
	///
	async fn set_friend_apply(&self, _action: SetFriendApplyType) -> Result<()> {
		Err("此接口未实现".into())
	}

	/// 设置加群申请
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `action` - 设置或拒绝加群申请
	///
	async fn set_invited_join_group(
		&self,
		_group_id: &str,
		_action: SetGroupApplyType,
	) -> Result<()> {
		Err("此接口未实现".into())
	}

	/// 上传文件
	///
	/// ## 参数
	/// `contact` - 联系人
	/// `file` - 文件路径
	/// `folder` - 目标文件夹
	async fn upload_file(
		&self,
		_contact: Contact,
		_file: Vec<u8>,
		_folder: Option<&str>,
	) -> Result<()> {
		Err("此接口未实现".into())
	}

	/// 下载文件到协议端
	///     NapCat之类的所在的
	///
	/// ## 参数
	/// `file` - 文件标识符
	/// `path` - 本地保存路径
	///
	async fn download_file(&self, _file: &str, _path: PathBuf) -> Result<DownloadFileInfo> {
		Err("此接口未实现".into())
	}

	/// 创建群文件目录
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `folder` - 目录名称
	///
	async fn create_group_folder(
		&self,
		_group_id: &str,
		_folder: &str,
	) -> Result<CreateGroupFolderInfo> {
		Err("此接口未实现".into())
	}

	/// 重命名群文件目录
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `folder_id` - 目录ID
	/// `folder_name` - 目录新名称
	///
	async fn rename_group_folder(
		&self,
		_group_id: &str,
		_folder_id: &str,
		_folder_name: &str,
	) -> Result<bool> {
		Err("此接口未实现".into())
	}

	/// 删除群文件目录
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `folder_id` - 目录ID
	///
	async fn delete_group_folder(&self, _group_id: &str, _folder_id: &str) -> Result<bool> {
		Err("此接口未实现".into())
	}

	/// 获取文件URL
	///
	/// ## 参数
	/// `contact` - 联系人
	/// `file_id` - 文件ID
	///
	async fn get_file_url(&self, _contact: Contact, _file_id: &str) -> Result<bool> {
		Err("此接口未实现".into())
	}

	/// 删除群文件
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `file_id` - 文件ID
	///
	async fn del_group_file(&self, _group_id: &str, _file_id: &str) -> Result<bool> {
		Err("此接口未实现".into())
	}

	/// 获取群文件系统信息
	///
	/// ## 参数
	/// `group_id` - 群ID
	///
	async fn get_group_file_system_info(&self, _group_id: &str) -> Result<QQGroupFileSystemInfo> {
		Err("此接口未实现".into())
	}

	/// 获取群文件列表
	///
	/// ## 参数
	/// `group_id` - 群ID
	/// `folder_id` - 目录ID，默认根目录
	///
	async fn get_group_file_list(
		&self,
		_group_id: &str,
		_folder_id: Option<&str>,
	) -> Result<Vec<DownloadFileInfo>> {
		Err("此接口未实现".into())
	}

	/// 获取群禁言列表
	///
	/// ## 参数
	/// `group_id` - 群ID
	///
	async fn get_group_mute_list(&self, _group_id: &str) -> Result<Vec<GroupMuteInfo>> {
		Err("此接口未实现".into())
	}

	/// 戳一戳
	///     支持群聊，私聊场景
	///
	/// ## 参数
	/// `contact` - 联系人
	/// `count` - 戳一戳次数，默认1次
	///
	async fn poke_user(&self, _contact: Contact, _count: Option<u8>) -> Result<bool> {
		Err("此接口未实现".into())
	}

	/// 设置头像
	///
	/// ## 参数
	/// `avatar` - 头像二进制数据
	///
	async fn set_avatar(&self, _avatar: Vec<u8>) -> Result<bool> {
		Err("此接口未实现".into())
	}

	/// 获取Cookie
	///   支持获取指定域名下的Cookie
	async fn get_cookie(&self, _domain: &str) -> Result<String> {
		Err("此接口未实现".into())
	}

	/// 获取QQ 相关接口凭证
	///   支持获取指定域名下的相关凭证
	async fn get_credentials(&self, _domain: &str) -> Result<QQCredentialInfo> {
		Err("此接口未实现".into())
	}

	/// 获取CSRF Token
	async fn get_csrf_token(&self) -> Result<u64> {
		Err("此接口未实现".into())
	}

	async fn get_rkey(&self) -> Result<QQRkeyInfo> {
		Err("此接口未实现".into())
	}
}

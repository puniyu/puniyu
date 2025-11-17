use crate::Result;
use puniyu_contact::ContactType;
use puniyu_element::Elements;
use puniyu_sender::{Role, SenderType, Sex};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AvatarSize {
	/// 小尺寸头像
	Small,
	/// 中等尺寸头像
	Medium,
	/// 大尺寸头像
	Large,
}

#[derive(Debug, Clone)]
pub struct Avatar(String);

impl Avatar {
	/// 将头像 URL 转换为字节数组
	///
	/// 支持 `file://` 协议和 `HTTP(S)协议` URL
	pub async fn to_vec(&self) -> Result<Vec<u8>> {
		Ok(reqwest::get(&self.0).await?.bytes().await?.to_vec())
	}
}

impl From<String> for Avatar {
	fn from(url: String) -> Self {
		Self(url)
	}
}

impl From<&str> for Avatar {
	fn from(url: &str) -> Self {
		Self(url.to_string())
	}
}

impl AsRef<str> for Avatar {
	fn as_ref(&self) -> &str {
		&self.0
	}
}

impl std::ops::Deref for Avatar {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::fmt::Display for Avatar {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendMsgType {
	/// 消息ID
	pub message_id: String,
	/// 发送时间戳(秒)
	pub time: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageType {
	/// 消息发送时间戳(秒)
	pub time: u64,
	/// 消息ID
	pub message_id: String,
	/// 消息序列号
	pub message_seq: u64,
	/// 消息联系人
	pub contact: ContactType,
	/// 消息发送者
	pub sender: SenderType,
	/// 消息元素
	pub elements: Vec<Elements>,
}

#[derive(Debug, Clone)]
pub enum HighlightsAction {
	/// 添加精华
	Add,
	/// 移除精华
	Remove,
}

#[derive(Debug, Clone)]
pub enum MuteType {
	/// 禁言
	Set,
	/// 解除禁言
	Remove,
}

#[derive(Debug, Clone)]
pub enum SetAdminType {
	/// 设置管理员
	Set,
	/// 取消管理员
	Remove,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupHighlightsType {
	/// 消息发送时间
	pub message_time: u64,
	/// 消息ID
	pub message_id: String,
	/// 消息序列号
	pub message_seq: u64,
	/// 群ID
	pub group_id: String,
	/// 消息发送者ID
	pub sender_id: String,
	/// 发送者昵称
	pub sender_name: String,
	/// 操作者ID
	pub target_id: String,
	/// 操作者昵称
	pub target_name: String,
	/// 操作时间
	pub target_time: u64,
	/// 被设置的精华消息元素文本
	pub json_elements: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfo {
	/// 用户ID
	pub user_id: String,
	/// 用户昵称
	pub nick: String,
	/// 用户头像
	pub avatar: String,
	/// 用户年龄
	pub age: Option<u8>,
	/// 用户性别
	pub sex: Sex,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupInfo {
	/// 群ID
	pub group_id: String,
	/// 群名称
	pub group_name: String,
	/// 群头像
	pub avatar: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupMemberInfo {
	/// 用户ID
	pub user_id: String,
	/// 用户昵称
	pub nick: String,
	/// 用户角色
	pub role: Role,
	/// 群头衔, 如果为空则无
	pub title: Option<String>,
	/// 群名片, 如果为空则与昵称相同
	pub card: String,
	/// 用户头像
	pub avatar: String,
	/// 用户年龄
	pub age: Option<u8>,
	/// 用户性别
	pub sex: Sex,
	/// 加入时间戳(秒)
	pub joined_at: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QQHonorInfo {
	/// 荣誉用户ID
	pub user_id: String,
	/// 荣誉用户昵称
	pub nick: String,
	/// 群荣誉ID
	pub honor_id: u64,
	/// 群荣誉名称
	pub honor_name: String,
	/// 群荣誉头像
	pub honor_avatar: String,
	/// 群荣誉描述
	pub honor_desc: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QQGroupFileSystemInfo {
	/// 文件数量
	pub file_count: u32,
	/// 文件上限
	pub limit_count: u32,
	/// 已使用空间
	pub used_space: u64,
	/// 空间上限
	pub total_space: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QQGroupFileInfo {
	/// 文件ID
	pub fid: String,
	/// 文件名
	pub name: String,
	/// 文件大小
	pub size: u64,
	/// 上传时间
	pub upload_time: u64,
	/// 过期时间
	pub expire_time: u64,
	/// 修改时间
	pub modify_time: u64,
	/// 下载次数
	pub download_count: u32,
	/// 上传者ID
	pub upload: String,
	/// 上传者昵称
	pub upload_name: String,
	/// MD5
	pub md5: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QQGroupFolderInfo {
	/// 文件夹ID
	pub id: String,
	/// 文件夹名
	pub name: String,
	/// 文件数量
	pub file_count: u32,
	/// 创建时间
	pub create_time: u64,
	/// 创建者ID
	pub creator_id: String,
	/// 创建者昵称
	pub creator_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QQGroupFileList {
	/// 文件列表
	pub files: Vec<QQGroupFileInfo>,
	/// 文件夹列表
	pub folders: Vec<QQGroupFolderInfo>,
}
#[derive(Debug, Clone)]
pub enum SetFriendApplyType {
	/// 同意好友申请
	Agree,
	/// 拒绝好友申请
	Refuse,
}

#[derive(Debug, Clone)]
pub enum SetGroupApplyType {
	/// 同意入群申请
	Agree,
	/// 拒绝入群申请
	Refuse,
}

#[derive(Debug, Clone)]
pub struct DownloadFileInfo {
	/// 文件
	pub file_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct CreateGroupFolderInfo {
	/// 文件夹ID
	pub folder_id: String,
	/// 文件夹名
	pub folder_name: String,
}

#[derive(Debug, Clone)]
pub struct GroupMuteInfo {
	/// 用户ID
	pub user_id: String,
	/// 禁言时间(秒)
	pub mute_time: u64,
}

#[derive(Debug, Clone)]
pub struct QQCredentialInfo {
	pub cookie: String,
	pub csrf_token: String,
}

#[derive(Debug, Clone)]
pub struct QQRkeyInfo {
	/// 可使用的场景
	pub r#type: RKeyType,
	/// rkey
	pub rkey: String,
	/// 创建时间
	pub created_at: u64,
	/// 过期时间
	pub ttl: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RKeyType {
	/// 私聊场景
	Private,
	/// 群聊场景
	Group,
}

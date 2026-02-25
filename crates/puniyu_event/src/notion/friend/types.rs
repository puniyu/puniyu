use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FriendAddType {}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FriendDecreaseType {}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReceiveLikeType {
	pub count: u8,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrivatePokeType {
	/// 目标id, 就是被戳的用户
	pub target_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrivateRecallType {
	pub message_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrivateFileUploadType {
	/// 文件id
	pub file_id: String,
	/// 文件名
	pub file_name: String,
	/// 文件大小
	pub file_size: u64,
	/// 文件Url
	pub file_url: String,
}


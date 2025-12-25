use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReceiveLikeOption {
	pub count: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrivatePokeOption {
	/// 目标id, 就是被戳的用户
	pub target_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrivateRecallOption {
	pub message_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrivateFileUploadOption {
	/// 文件id
	pub file_id: String,
	/// 文件名
	pub file_name: String,
	/// 文件大小
	pub file_size: u64,
	/// 文件Url
	pub file_url: String,
}

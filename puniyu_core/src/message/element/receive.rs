use crate::message::element::send::{
	JsonElement, MusicElement, RecordElement, ReplyElement, TextElement, XmlElement,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveAtElement {
	/// at元素类型
	#[serde(rename = "type")]
	pub r#type: String,
	/// at元素目标id
	pub target_id: String,
	/// at元素目标名称
	pub name: Option<String>,
}

impl ReceiveAtElement {
	/// 是否为艾特全体
	pub fn is_all(&self) -> bool {
		self.target_id.contains("all")
	}
	/// 是否为艾特在线成员
	pub fn is_online(&self) -> bool {
		self.target_id.contains("online")
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveFaceElement {
	/// 元素类型
	#[serde(rename = "type")]
	pub r#type: String,
	/// 表情元素id
	pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveFileElement {
	/// 元素类型
	#[serde(rename = "type")]
	pub r#type: String,
	/// 文件元素id
	///  - 文件网络url
	///  - 文件绝对路径
	///  - base64
	pub file: String,
	/// 文件id
	pub file_id: String,
	/// 文件大小, 单位字节
	pub file_size: u64,
	/// 文件名称
	pub file_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveImageElement {
	/// 元素类型
	#[serde(rename = "type")]
	pub r#type: String,
	/// 图片元素id
	///  - 图片网络url
	///  - 图片绝对路径
	///  - base64
	pub file: String,
	/// 是否为闪照
	pub is_flash: bool,
	/// 图片外显
	pub summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveVideoElement {
	/// 元素类型
	#[serde(rename = "type")]
	pub r#type: String,
	/// 视频元素id
	///  - 视频网络url
	///  - 视频绝对路径
	///  - base64
	pub file: String,
	/// 视频文件名
	pub file_name: String,
}

pub enum ReceiveElements {
	At(ReceiveAtElement),
	Image(ReceiveImageElement),
	Json(JsonElement),
	Music(MusicElement),
	Record(RecordElement),
	Reply(ReplyElement),
	Text(TextElement),
	Video(ReceiveVideoElement),
	Xml(XmlElement),
}

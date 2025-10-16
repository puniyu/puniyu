use super::{RawMessage, TextMessage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtElement {
	/// at元素类型
	#[serde(rename = "type")]
	pub r#type: String,
	/// at元素目标id
	pub target_id: String,
	/// at元素目标名称
	pub name: Option<String>,
}

impl AtElement {
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
pub struct FaceElement {
	/// 元素类型
	#[serde(rename = "type")]
	pub r#type: String,
	/// 表情元素id
	pub id: u64,
}

impl RawMessage for FaceElement {
	fn raw(&self) -> String {
		format!("[face:{}]", self.id)
	}
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileElement {
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

impl RawMessage for FileElement {
	fn raw(&self) -> String {
		if self.file.starts_with("http") || self.file.starts_with("file") {
			format!("[file:{}, fid:{}]", self.file, self.file_id)
		} else if self.file.starts_with("base64://") {
			format!("[file:base64://..., fid:{}", self.file_id)
		} else {
			format!("[file:{}, fid:{}]", self.file, self.file_id)
		}
	}
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageElement {
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

impl RawMessage for ImageElement {
	fn raw(&self) -> String {
		if self.file.starts_with("http") || self.file.starts_with("file") {
			format!("[image:{}]", self.file)
		} else {
			String::from("[image:base64://...]")
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonElement {
	/// json元素类型
	#[serde(rename = "type")]
	pub r#type: String,
	/// Json数据，未序列化
	pub data: serde_json::Value,
}

impl RawMessage for JsonElement {
	fn raw(&self) -> String {
		format!("[json:{}]", self.data)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoElement {
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

impl RawMessage for VideoElement {
	fn raw(&self) -> String {
		if self.file.starts_with("http") || self.file.starts_with("file") {
			format!("[video:{}]", self.file)
		} else {
			String::from("[video:base64://...]")
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordElement {
	/// 元素类型
	#[serde(rename = "type")]
	pub r#type: String,
	/// 语言元素
	///  - 语音网络url
	///  - 语音绝对路径
	///  - base64
	pub file: String,
}

impl RawMessage for RecordElement {
	fn raw(&self) -> String {
		if self.file.starts_with("http") || self.file.starts_with("file") {
			format!("[record:{}]", self.file)
		} else {
			String::from("[record:base64://...]")
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyElement {
	/// 元素类型
	#[serde(rename = "type")]
	pub r#type: String,
	/// 回复元素id
	#[serde(rename = "messageId")]
	pub message_id: String,
}

impl RawMessage for ReplyElement {
	fn raw(&self) -> String {
		format!("[reply:{}]", self.message_id)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextElement {
	/// 文本元素类型
	#[serde(rename = "type")]
	pub r#type: String,
	/// 文本元素内容
	pub text: String,
}

impl RawMessage for TextElement {
	fn raw(&self) -> String {
		format!("[text:{}]", self.text)
	}
}

impl TextMessage for TextElement {
	fn text(&self) -> String {
		self.text.clone()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XmlElement {
	/// xml元素类型
	#[serde(rename = "type")]
	pub r#type: String,
	/// Xml数据，未序列化
	pub data: String,
}

impl RawMessage for XmlElement {
	fn raw(&self) -> String {
		format!("[xml:{}]", self.data)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Elements {
	At(AtElement),
	Image(ImageElement),
	Json(JsonElement),
	Record(RecordElement),
	Reply(ReplyElement),
	Text(TextElement),
	Video(VideoElement),
	Xml(XmlElement),
}

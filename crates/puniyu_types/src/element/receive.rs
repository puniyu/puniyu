use super::RawMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElement {
	/// 文本元素内容
	pub text: String,
}
impl From<TextElement> for String {
	fn from(elem: TextElement) -> Self {
		elem.text
	}
}

impl RawMessage for TextElement {
	fn raw(&self) -> String {
		format!("[text:{}]", self.text)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtElement {
	/// at元素目标id
	pub target_id: String,
}

impl AtElement {
	/// 是否为艾特全体
	pub fn is_all(&self) -> bool {
		matches!(self.target_id.as_str(), "all")
	}
}

impl RawMessage for AtElement {
	fn raw(&self) -> String {
		format!("[at:{}]", self.target_id)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyElement {
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
pub struct FaceElement {
	/// 表情元素id
	pub id: u64,
}

impl RawMessage for FaceElement {
	fn raw(&self) -> String {
		format!("[face:{}]", self.id)
	}
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageElement {
	/// 图片元素
	pub file: Vec<u8>,
	/// 图片外显
	pub summary: String,
	/// 图片宽度
	pub width: u64,
	/// 图片高度
	pub height: u64,
}

impl RawMessage for ImageElement {
	fn raw(&self) -> String {
		format!("[image:{}]", self.summary.clone())
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileElement {
	/// 文件元素
	pub file: Vec<u8>,
	/// 文件id
	pub file_id: String,
	/// 文件大小, 单位字节
	pub file_size: u64,
	/// 文件名称
	pub file_name: String,
}

impl RawMessage for FileElement {
	fn raw(&self) -> String {
		format!("[file:{}]", self.file_id)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoElement {
	/// 视频元素
	pub file: Vec<u8>,
	/// 视频文件名
	pub file_name: String,
}

impl RawMessage for VideoElement {
	fn raw(&self) -> String {
		format!("[video:{}]", self.file_name)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordElement {
	/// 语言元素
	pub file: Vec<u8>,
}

impl RawMessage for RecordElement {
	fn raw(&self) -> String {
		format!("[record:{}]", "语音消息")
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonElement {
	/// Json数据，未序列化
	pub data: String,
}

impl RawMessage for JsonElement {
	fn raw(&self) -> String {
		format!("[json:{}]", self.data)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlElement {
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
	Text(TextElement),
	At(AtElement),
	Reply(ReplyElement),
	Face(FaceElement),
	Image(ImageElement),
	File(FileElement),
	Video(VideoElement),
	Record(RecordElement),
	Json(JsonElement),
	Xml(XmlElement),
}

impl Elements {
	pub fn as_text(&self) -> Option<&str> {
		match self {
			Elements::Text(text_element) => Some(&text_element.text),
			_ => None,
		}
	}

	pub fn as_at(&self) -> Option<&AtElement> {
		match self {
			Elements::At(at_element) => Some(at_element),
			_ => None,
		}
	}
}

impl RawMessage for Elements {
	fn raw(&self) -> String {
		match self {
			Elements::Text(text_element) => text_element.raw(),
			Elements::File(file_element) => file_element.raw(),
			Elements::Face(face_element) => face_element.raw(),
			Elements::Image(image_element) => image_element.raw(),
			Elements::Json(json_element) => json_element.raw(),
			Elements::Record(record_element) => record_element.raw(),
			Elements::Reply(reply_element) => reply_element.raw(),
			Elements::Video(video_element) => video_element.raw(),
			Elements::Xml(xml_element) => xml_element.raw(),
			Elements::At(at_element) => at_element.raw(),
		}
	}
}

impl RawMessage for Vec<Elements> {
	fn raw(&self) -> String {
		self.iter().map(|element| element.raw()).collect::<Vec<_>>().join("")
	}
}

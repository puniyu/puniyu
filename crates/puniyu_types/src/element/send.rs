use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AtElement {
	/// at元素目标id
	pub target_id: String,
}

impl AtElement {
	pub fn new(target_id: String) -> Self {
		Self { target_id }
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceElement {
	/// 表情元素id
	pub id: u64,
}

impl FaceElement {
	pub fn new(id: impl Into<u64>) -> Self {
		Self { id: id.into() }
	}
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileElement {
	/// 文件元素
	pub file: Vec<u8>,
}

impl FileElement {
	pub fn new(file: Vec<u8>) -> Self {
		Self { file }
	}
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageElement {
	/// 图片元素
	pub file: Vec<u8>,
	/// 是否为闪照
	pub is_flash: bool,
	/// 图片外显
	pub summary: Option<String>,
}

impl ImageElement {
	pub fn new(file: Vec<u8>, is_flash: bool, summary: Option<String>) -> Self {
		Self { file, is_flash, summary }
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonElement {
	/// Json数据，未序列化
	pub data: String,
}

impl JsonElement {
	pub fn new(data: String) -> Self {
		Self { data }
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoElement {
	/// 视频元素
	pub file: Vec<u8>,
	/// 视频文件名
	pub file_name: String,
}

impl VideoElement {
	pub fn new(file: Vec<u8>, file_name: String) -> Self {
		Self { file, file_name }
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordElement {
	/// 语言元素
	pub file: Vec<u8>,
}

impl RecordElement {
	pub fn new(file: Vec<u8>) -> Self {
		Self { file }
	}
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyElement {
	/// 回复元素id
	pub message_id: String,
}

impl ReplyElement {
	pub fn new(message_id: String) -> Self {
		Self { message_id }
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElement {
	/// 文本元素内容
	pub text: String,
}

impl TextElement {
	pub fn new(text: String) -> Self {
		Self { text }
	}
}

impl From<String> for TextElement {
	fn from(text: String) -> Self {
		Self { text }
	}
}

impl From<&String> for TextElement {
	fn from(text: &String) -> Self {
		Self { text: text.clone() }
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlElement {
	/// Xml数据，未序列化
	pub data: String,
}

impl XmlElement {
	pub fn new(data: String) -> Self {
		Self { data }
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub enum Elements {
	At(AtElement),
	File(FileElement),
	Image(ImageElement),
	Json(JsonElement),
	Record(RecordElement),
	Reply(ReplyElement),
	Text(TextElement),
	Video(VideoElement),
	Xml(XmlElement),
}

impl Elements {
	pub fn as_text(&self) -> Option<&str> {
		match self {
			Elements::Text(text_element) => Some(&text_element.text),
			_ => None,
		}
	}
}

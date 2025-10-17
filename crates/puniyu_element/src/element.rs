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

impl RawMessage for AtElement {
	fn raw(&self) -> String {
		format!("[at:{}]", self.target_id)
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

impl Elements {
	pub fn as_text(&self) -> Option<&str> {
		match self {
			Elements::Text(text_element) => Some(&text_element.text),
			_ => None,
		}
	}
}

impl RawMessage for Elements {
	fn raw(&self) -> String {
		match self {
			Elements::Text(text_element) => text_element.raw(),
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
#[macro_export]
macro_rules! element {
	// 文本元素
	(text,$text:expr) => {
		Elements::Text(TextElement {
			r#type: ElementType::Text.to_string(),
			text: $text.to_string(),
		})
	};

	// 图片元素
	(image $file:expr) => {
		$crate::element::Elements::Image($crate::element::ImageElement {
			r#type: "image".to_string(),
			file: $file.to_string(),
			is_flash: false,
			summary: None,
		})
	};

	// @元素
	(at $target_id:expr) => {
		$crate::element::Elements::At($crate::element::AtElement {
			r#type: "at".to_string(),
			target_id: $target_id.to_string(),
			name: None,
		})
	};

	// @全体成员
	(at_all) => {
		$crate::element::Elements::At($crate::element::AtElement {
			r#type: "at".to_string(),
			target_id: "all".to_string(),
			name: Some("全体成员".to_string()),
		})
	};

	// 表情元素
	(face $id:expr) => {
		$crate::element::Elements::Face($crate::element::FaceElement {
			r#type: "face".to_string(),
			id: $id,
		})
	};

	// 回复元素
	(reply $message_id:expr) => {
		$crate::element::Elements::Reply($crate::element::ReplyElement {
			r#type: "reply".to_string(),
			message_id: $message_id.to_string(),
		})
	};

	// 语音元素
	(record $file:expr) => {
		$crate::element::Elements::Record($crate::element::RecordElement {
			r#type: "record".to_string(),
			file: $file.to_string(),
		})
	};

	// 视频元素
	(video $file:expr, $file_name:expr) => {
		$crate::element::Elements::Video($crate::element::VideoElement {
			r#type: "video".to_string(),
			file: $file.to_string(),
			file_name: $file_name.to_string(),
		})
	};

	// JSON元素
	(json $data:expr) => {
		$crate::element::Elements::Json($crate::element::JsonElement {
			r#type: "json".to_string(),
			data: serde_json::Value::String($data.to_string()),
		})
	};

	// XML元素
	(xml $data:expr) => {
		$crate::element::Elements::Xml($crate::element::XmlElement {
			r#type: "xml".to_string(),
			data: $data.to_string(),
		})
	};
}
